use crate::core::{
    state::{Pool, BalanceDelta, Result as StateResult, PositionKey},
    hooks::{
        Hook, HookRegistry, BeforeHookResult, AfterHookResult, HookResult, HookError,
        hook_interface::{PoolKey, ModifyLiquidityParams},
    },
};

use ethers::types::Address;
use super::{Result, PoolError};

/// Modify liquidity in the pool (add or remove)
pub fn modify_liquidity(
    pool: &mut Pool,
    key: &PoolKey,
    params: &ModifyLiquidityParams,
    hook_registry: &mut HookRegistry,
    sender: Address,
    hook_data: &[u8],
) -> Result<(BalanceDelta, BalanceDelta)> {
    // Call hook before modifying liquidity if available
    let hook_address = Address::from_slice(&key.hooks);
    if hook_address != Address::zero() {
        if let Some(hook) = hook_registry.get_hook_mut(&key.hooks) {
            let hook_result = if params.liquidity_delta > 0 {
                hook.before_add_liquidity(
                    sender.0,
                    key,
                    params,
                    hook_data
                )
            } else {
                hook.before_remove_liquidity(
                    sender.0,
                    key,
                    params,
                    hook_data
                )
            };
            
            // Handle hook result
            if let Err(e) = hook_result {
                return Err(PoolError::StateError(e));
            }
        }
    }
    
    // Modify liquidity in the pool
    let (principal_delta, fees_accrued) = pool.modify_liquidity(
        params.tick_lower,
        params.tick_upper,
        params.liquidity_delta,
        key.tick_spacing
    ).map_err(PoolError::StateError)?;
    
    // Combine principal delta and fees for the caller
    let mut caller_delta = principal_delta + fees_accrued;
    
    // Call hook after modifying liquidity if available
    let mut hook_delta = BalanceDelta::default();
    if hook_address != Address::zero() {
        if let Some(hook) = hook_registry.get_hook_mut(&key.hooks) {
            let hook_result = if params.liquidity_delta > 0 {
                hook.after_add_liquidity(
                    sender.0,
                    key,
                    params,
                    &caller_delta,
                    &fees_accrued,
                    hook_data
                )
            } else {
                hook.after_remove_liquidity(
                    sender.0,
                    key,
                    params,
                    &caller_delta,
                    &fees_accrued,
                    hook_data
                )
            };
            
            // Handle hook result
            match hook_result {
                Ok(AfterHookResult { delta: Some(delta) }) => {
                    hook_delta = delta;
                },
                Ok(_) => {},
                Err(e) => return Err(PoolError::StateError(e)),
            }
        }
    }
    
    Ok((caller_delta, fees_accrued))
} 