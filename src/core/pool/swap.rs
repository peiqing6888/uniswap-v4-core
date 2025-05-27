use crate::core::{
    math::types::SqrtPrice,
    state::{Pool, BalanceDelta, Result as StateResult},
    hooks::{
        Hook, HookRegistry, BeforeHookResult, AfterHookResult, BeforeSwapDelta,
        hook_interface::{PoolKey, SwapParams},
    },
};

use ethers::types::Address;
use primitive_types::U256;
use super::{Result, PoolError};

/// Execute a swap on the pool
pub fn swap(
    pool: &mut Pool,
    key: &PoolKey,
    params: &SwapParams,
    hook_registry: &mut HookRegistry,
    sender: Address,
    hook_data: &[u8],
) -> Result<BalanceDelta> {
    // Check that amount specified is not zero
    if params.amount_specified == 0 {
        return Err(PoolError::SwapAmountCannotBeZero);
    }
    
    // Call hook before swap if available
    let mut amount_to_swap = params.amount_specified;
    let mut before_swap_delta = BeforeSwapDelta::default();
    let mut lp_fee_override = None;
    
    let hook_address = Address::from_slice(&key.hooks);
    if hook_address != Address::zero() {
        if let Some(hook) = hook_registry.get_hook_mut(&key.hooks) {
            let hook_result = hook.before_swap(
                sender.0,
                key,
                params,
                hook_data
            ).map_err(PoolError::HookError)?;
            
            if let BeforeHookResult { amount: Some(amount), delta: Some(delta), fee_override } = hook_result {
                amount_to_swap = amount;
                before_swap_delta = delta;
                lp_fee_override = fee_override;
            }
        }
    }
    
    // Execute swap
    let (swap_delta, protocol_fee) = pool.swap(
        amount_to_swap,
        params.sqrt_price_limit_x96,
        params.zero_for_one,
        key.tick_spacing,
    ).map_err(PoolError::StateError)?;
    
    // Call hook after swap if available
    let mut hook_delta = BalanceDelta::default();
    let result_delta = swap_delta;
    
    if hook_address != Address::zero() {
        if let Some(hook) = hook_registry.get_hook_mut(&key.hooks) {
            let hook_result = hook.after_swap(
                sender.0,
                key,
                params,
                &swap_delta,
                hook_data
            ).map_err(PoolError::HookError)?;
            
            if let AfterHookResult { delta: Some(delta) } = hook_result {
                hook_delta = delta;
            }
        }
    }
    
    Ok(result_delta)
} 