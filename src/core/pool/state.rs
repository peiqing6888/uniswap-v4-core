use crate::core::{
    math::types::SqrtPrice,
    state::{Pool, Result as StateResult},
};

use super::{Result, PoolError};

/// Get the current tick of a pool
pub fn get_tick(
    pool: &Pool,
) -> Result<i32> {
    Ok(pool.tick)
}

/// Get the current sqrt price of a pool
pub fn get_sqrt_price(
    pool: &Pool,
) -> Result<SqrtPrice> {
    Ok(pool.sqrt_price_x96)
}

/// Get the current liquidity of a pool
pub fn get_liquidity(
    pool: &Pool,
) -> Result<u128> {
    Ok(pool.liquidity)
}

/// Get the fee growth globals of a pool
pub fn get_fee_growth_globals(
    pool: &Pool,
) -> Result<(u128, u128)> {
    Ok((pool.fee_growth_global_0_x128, pool.fee_growth_global_1_x128))
}

/// Get the protocol fees of a pool
pub fn get_protocol_fees(
    pool: &Pool,
) -> Result<(u128, u128)> {
    Ok((pool.protocol_fee_0, pool.protocol_fee_1))
}

/// Get the LP fee of a pool
pub fn get_lp_fee(
    pool: &Pool,
) -> Result<u32> {
    Ok(pool.fee)
}

/// Check if a pool is initialized
pub fn is_initialized(
    pool: &Pool,
) -> bool {
    // In the Solidity implementation, this would check if the pool exists
    // In our Rust implementation, we can check if the sqrt price is non-zero
    !pool.sqrt_price_x96.is_zero()
} 