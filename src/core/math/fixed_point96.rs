use primitive_types::U256;
use std::ops::{Add, Sub, Mul, Div};
use num_traits::Zero;

/// Fixed point Q96 operations
pub struct FixedPoint96;

/// Q96 scale - 2^96
pub const Q96: U256 = U256([0, 1, 0, 0]);

impl FixedPoint96 {
    /// Multiplies two Q96 numbers and returns a Q96 number
    pub fn mul(a: U256, b: U256) -> U256 {
        a.saturating_mul(b) / Q96
    }
    
    /// Divides two numbers and returns a Q96 number
    pub fn div(a: U256, b: U256) -> U256 {
        a.saturating_mul(Q96) / b
    }
    
    /// Multiply a U256 by a uint128, returning a U256
    pub fn mul_div(a: U256, b: U256, denominator: U256) -> U256 {
        a.saturating_mul(b) / denominator
    }
    
    /// Convert a U256 to an i128
    pub fn to_i128(x: U256) -> i128 {
        let u128_value = x.as_u128();
        if u128_value > i128::MAX as u128 {
            i128::MAX
        } else {
            u128_value as i128
        }
    }
    
    /// Returns the amount of token0 for a given amount of liquidity and a price range
    pub fn get_amount0_for_liquidity(
        sqrt_price_a_x96: U256,
        sqrt_price_b_x96: U256,
        liquidity: u128,
    ) -> u128 {
        let (sqrt_price_lower, sqrt_price_upper) = if sqrt_price_a_x96 <= sqrt_price_b_x96 {
            (sqrt_price_a_x96, sqrt_price_b_x96)
        } else {
            (sqrt_price_b_x96, sqrt_price_a_x96)
        };
        
        let numerator1 = U256::from(liquidity) << 96;
        let numerator2 = sqrt_price_upper - sqrt_price_lower;
        
        // This is safe because sqrt_price_upper > sqrt_price_lower
        ((numerator1 * numerator2 / sqrt_price_upper) / sqrt_price_lower).as_u128()
    }
    
    /// Returns the amount of token1 for a given amount of liquidity and a price range
    pub fn get_amount1_for_liquidity(
        sqrt_price_a_x96: U256,
        sqrt_price_b_x96: U256,
        liquidity: u128,
    ) -> u128 {
        let (sqrt_price_lower, sqrt_price_upper) = if sqrt_price_a_x96 <= sqrt_price_b_x96 {
            (sqrt_price_a_x96, sqrt_price_b_x96)
        } else {
            (sqrt_price_b_x96, sqrt_price_a_x96)
        };
        
        ((U256::from(liquidity) * (sqrt_price_upper - sqrt_price_lower)) >> 96).as_u128()
    }
    
    /// Returns the liquidity amount for given amounts of token0 and token1
    pub fn get_liquidity_for_amounts(
        sqrt_price_current_x96: U256,
        sqrt_price_a_x96: U256,
        sqrt_price_b_x96: U256,
        amount0: u128,
        amount1: u128,
    ) -> u128 {
        let (sqrt_price_lower, sqrt_price_upper) = if sqrt_price_a_x96 <= sqrt_price_b_x96 {
            (sqrt_price_a_x96, sqrt_price_b_x96)
        } else {
            (sqrt_price_b_x96, sqrt_price_a_x96)
        };
        
        let mut liquidity: u128 = 0;
        
        if sqrt_price_current_x96 <= sqrt_price_lower {
            // Current price is below the provided range, liquidity all in token0
            liquidity = Self::get_liquidity_for_amount0(
                sqrt_price_lower,
                sqrt_price_upper,
                amount0,
            );
        } else if sqrt_price_current_x96 < sqrt_price_upper {
            // Current price is in the range, liquidity in both tokens
            let liquidity0 = Self::get_liquidity_for_amount0(
                sqrt_price_current_x96,
                sqrt_price_upper,
                amount0,
            );
            let liquidity1 = Self::get_liquidity_for_amount1(
                sqrt_price_lower,
                sqrt_price_current_x96,
                amount1,
            );
            
            // Use the smaller liquidity
            liquidity = if liquidity0 < liquidity1 { liquidity0 } else { liquidity1 };
        } else {
            // Current price is above the provided range, liquidity all in token1
            liquidity = Self::get_liquidity_for_amount1(
                sqrt_price_lower,
                sqrt_price_upper,
                amount1,
            );
        }
        
        liquidity
    }
    
    /// Returns the liquidity amount for a given amount of token0
    pub fn get_liquidity_for_amount0(
        sqrt_price_lower_x96: U256,
        sqrt_price_upper_x96: U256,
        amount0: u128,
    ) -> u128 {
        // Amount0 is stored in first 128 bits
        let amount0 = U256::from(amount0);
        let numerator = amount0 * sqrt_price_lower_x96 * sqrt_price_upper_x96;
        let denominator = (sqrt_price_upper_x96 - sqrt_price_lower_x96) << 96;
        
        (numerator / denominator).as_u128()
    }
    
    /// Returns the liquidity amount for a given amount of token1
    pub fn get_liquidity_for_amount1(
        sqrt_price_lower_x96: U256,
        sqrt_price_upper_x96: U256,
        amount1: u128,
    ) -> u128 {
        // Amount1 is stored in first 128 bits
        let amount1 = U256::from(amount1);
        let numerator = amount1 << 96;
        let denominator = sqrt_price_upper_x96 - sqrt_price_lower_x96;
        
        (numerator / denominator).as_u128()
    }
} 