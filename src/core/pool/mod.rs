use crate::core::{
    math::{
        types::{SqrtPrice, Liquidity},
        tick_math::TickMath,
        sqrt_price_math::SqrtPriceMath,
        swap_math::SwapMath,
    },
    state::{
        Pool, BalanceDelta, StateError, Result as StateResult,
    },
    hooks::{
        Hook, HookRegistry, HookFlags,
        hook_interface::{PoolKey, SwapParams, ModifyLiquidityParams},
    },
};

use ethers::types::Address;
use primitive_types::U256;

/// Pool management functions
// pub mod management;

/// Pool swap functions
// pub mod swap;

/// Pool liquidity functions
// pub mod liquidity;

/// Pool initialization functions
// pub mod initialize;

/// Pool fee functions
// pub mod fees;

/// Pool state access functions
// pub mod state;

/// Error type for pool operations
#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    #[error("State error: {0}")]
    StateError(#[from] StateError),
    
    #[error("Hook error: {0}")]
    HookError(#[from] crate::core::hooks::HookError),
    
    #[error("Invalid fee tier")]
    InvalidFeeTier,
    
    #[error("Currencies out of order: token0 {0:?}, token1 {1:?}")]
    CurrenciesOutOfOrderOrEqual(Address, Address),
    
    #[error("Tick spacing too large: {0}")]
    TickSpacingTooLarge(i32),
    
    #[error("Tick spacing too small: {0}")]
    TickSpacingTooSmall(i32),
    
    #[error("Swap amount cannot be zero")]
    SwapAmountCannotBeZero,
    
    #[error("Currency not settled")]
    CurrencyNotSettled,
}

/// Result type for pool operations
pub type Result<T> = std::result::Result<T, PoolError>;

/// Constants for tick spacing
pub const MAX_TICK_SPACING: i32 = 16384;
pub const MIN_TICK_SPACING: i32 = 1;

/// Helper function to validate pool key
pub fn validate_pool_key(key: &PoolKey, hook_registry: &HookRegistry) -> Result<()> {
    // Check tick spacing
    if key.tick_spacing > MAX_TICK_SPACING {
        return Err(PoolError::TickSpacingTooLarge(key.tick_spacing));
    }
    if key.tick_spacing < MIN_TICK_SPACING {
        return Err(PoolError::TickSpacingTooSmall(key.tick_spacing));
    }
    
    // Check currencies are in order
    let token0 = Address::from_slice(&key.token0);
    let token1 = Address::from_slice(&key.token1);
    
    if token0 >= token1 {
        return Err(PoolError::CurrenciesOutOfOrderOrEqual(token0, token1));
    }
    
    // Check hook address is valid
    let hook_address = Address::from_slice(&key.hooks);
    if hook_address != Address::zero() {
        if let Some(hook) = hook_registry.get_hook(&key.hooks) {
            let hook_flags = HookFlags::from_address(key.hooks);
            if !hook_flags.validate_hook_address() {
                return Err(PoolError::HookError(crate::core::hooks::HookError::HookAddressNotValid(key.hooks)));
            }
        }
    }
    
    Ok(())
}

/// Helper function to get initial LP fee from fee parameter
pub fn get_initial_lp_fee(fee: u32) -> u32 {
    // In Solidity version, if the fee is a dynamic fee (highest bit set),
    // the initial LP fee is the lower 23 bits
    if crate::core::hooks::is_dynamic_fee(fee) {
        fee & 0x7FFFFF // Clear the highest bit
    } else {
        fee // Static fee
    }
}
