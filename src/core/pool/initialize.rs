use crate::core::{
    math::types::SqrtPrice,
    state::{Pool, Result as StateResult},
    hooks::{
        Hook, HookRegistry,
        hook_interface::PoolKey,
    },
};

use ethers::types::Address;
use super::{Result, PoolError, validate_pool_key, get_initial_lp_fee};

/// Initialize a pool with the given parameters
pub fn initialize_pool(
    pool: &mut Pool,
    key: &PoolKey,
    sqrt_price_x96: SqrtPrice,
    hook_registry: &mut HookRegistry,
    sender: Address,
) -> Result<i32> {
    // Validate pool key
    validate_pool_key(key, hook_registry)?;
    
    // Get initial LP fee
    let lp_fee = get_initial_lp_fee(key.fee);
    
    // Call hook before initialize if available
    let hook_address = Address::from_slice(&key.hooks);
    if hook_address != Address::zero() {
        if let Some(hook) = hook_registry.get_hook_mut(&key.hooks) {
            hook.before_initialize(
                sender.0,
                key,
                sqrt_price_x96,
                &[]  // Empty hook data
            ).map_err(PoolError::StateError)?;
        }
    }
    
    // Initialize pool
    let tick = pool.initialize(sqrt_price_x96, key.fee)
        .map_err(PoolError::StateError)?;
    
    // Call hook after initialize if available
    if hook_address != Address::zero() {
        if let Some(hook) = hook_registry.get_hook_mut(&key.hooks) {
            hook.after_initialize(
                sender.0,
                key,
                sqrt_price_x96,
                tick,
                &[]  // Empty hook data
            ).map_err(PoolError::StateError)?;
        }
    }
    
    Ok(tick)
} 