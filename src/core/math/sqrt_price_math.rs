use primitive_types::U256;
use crate::core::math::{
    types::{SqrtPrice, Liquidity, Q96},
    full_math::FullMath,
    MathError,
    Result,
};

/// Functions for handling square root price calculations
pub struct SqrtPriceMath;

impl SqrtPriceMath {
    /// Gets the amount0 delta between two prices
    pub fn get_amount0_delta(
        sqrt_price_a_x96: SqrtPrice,
        sqrt_price_b_x96: SqrtPrice,
        liquidity: Liquidity,
        round_up: bool,
    ) -> Result<U256> {
        if sqrt_price_a_x96.to_u256() > sqrt_price_b_x96.to_u256() {
            return Self::get_amount0_delta(sqrt_price_b_x96, sqrt_price_a_x96, liquidity, round_up);
        }

        let numerator1 = U256::from(liquidity.to_u128()) << 96;
        let numerator2 = sqrt_price_b_x96.to_u256() - sqrt_price_a_x96.to_u256();

        if sqrt_price_a_x96.to_u256().is_zero() {
            return Err(MathError::InvalidPrice);
        }

        if round_up {
            let product = sqrt_price_b_x96.to_u256() * sqrt_price_a_x96.to_u256();
            let result = FullMath::mul_div_rounding_up(
                numerator1,
                numerator2,
                product,
            ).ok_or(MathError::Overflow)?;
            Ok(result)
        } else {
            let product = sqrt_price_b_x96.to_u256() * sqrt_price_a_x96.to_u256();
            let result = FullMath::mul_div(
                numerator1,
                numerator2,
                product,
            ).ok_or(MathError::Overflow)?;
            Ok(result)
        }
    }

    /// Gets the amount1 delta between two prices
    pub fn get_amount1_delta(
        sqrt_price_a_x96: SqrtPrice,
        sqrt_price_b_x96: SqrtPrice,
        liquidity: Liquidity,
        round_up: bool,
    ) -> Result<U256> {
        if sqrt_price_a_x96.to_u256() > sqrt_price_b_x96.to_u256() {
            return Self::get_amount1_delta(sqrt_price_b_x96, sqrt_price_a_x96, liquidity, round_up);
        }

        let numerator = U256::from(liquidity.to_u128()) * (sqrt_price_b_x96.to_u256() - sqrt_price_a_x96.to_u256());
        let denominator = Q96;

        if round_up {
            FullMath::mul_div_rounding_up(
                numerator,
                U256::one(),
                denominator,
            ).ok_or(MathError::Overflow)
        } else {
            FullMath::mul_div(
                numerator,
                U256::one(),
                denominator,
            ).ok_or(MathError::Overflow)
        }
    }

    /// Gets the next sqrt price given a delta of token0
    pub fn get_next_sqrt_price_from_amount0_rounding_up(
        sqrt_price_x96: SqrtPrice,
        liquidity: Liquidity,
        amount: U256,
        add: bool,
    ) -> Result<SqrtPrice> {
        if amount.is_zero() {
            return Ok(sqrt_price_x96);
        }

        let numerator1 = U256::from(liquidity.to_u128()) << 96;

        if add {
            let product_result = amount.checked_mul(sqrt_price_x96.to_u256());
            if let Some(product) = product_result {
                let denominator = numerator1.checked_add(product).ok_or(MathError::Overflow)?;
                if denominator >= numerator1 {
                    return Ok(SqrtPrice::new(
                        FullMath::mul_div_rounding_up(numerator1, sqrt_price_x96.to_u256(), denominator)
                            .ok_or(MathError::Overflow)?,
                    ));
                }
            }

            Ok(SqrtPrice::new(
                numerator1 / (numerator1 / sqrt_price_x96.to_u256() + amount),
            ))
        } else {
            let product_result = amount.checked_mul(sqrt_price_x96.to_u256());
            
            if product_result.is_none() || numerator1 <= product_result.unwrap_or(U256::zero()) {
                return Err(MathError::PriceOverflow);
            }

            let product = product_result.unwrap();
            let denominator = numerator1.checked_sub(product).ok_or(MathError::Overflow)?;
            Ok(SqrtPrice::new(
                FullMath::mul_div_rounding_up(numerator1, sqrt_price_x96.to_u256(), denominator)
                    .ok_or(MathError::Overflow)?,
            ))
        }
    }

    /// Gets the next sqrt price given a delta of token1
    pub fn get_next_sqrt_price_from_amount1_rounding_down(
        sqrt_price_x96: SqrtPrice,
        liquidity: Liquidity,
        amount: U256,
        add: bool,
    ) -> Result<SqrtPrice> {
        if add {
            let quotient = if amount <= U256::from(u128::MAX) {
                (amount << 96) / U256::from(liquidity.to_u128())
            } else {
                FullMath::mul_div(amount, Q96, U256::from(liquidity.to_u128()))
                    .ok_or(MathError::Overflow)?
            };

            Ok(SqrtPrice::new(sqrt_price_x96.to_u256() + quotient))
        } else {
            let quotient = if amount <= U256::from(u128::MAX) {
                ((amount << 96) + U256::from(liquidity.to_u128()) - U256::one()) 
                    / U256::from(liquidity.to_u128())
            } else {
                FullMath::mul_div_rounding_up(amount, Q96, U256::from(liquidity.to_u128()))
                    .ok_or(MathError::Overflow)?
            };

            if sqrt_price_x96.to_u256() <= quotient {
                return Err(MathError::NotEnoughLiquidity);
            }

            Ok(SqrtPrice::new(sqrt_price_x96.to_u256() - quotient))
        }
    }

    /// Gets the next sqrt price given an input amount of token0 or token1
    pub fn get_next_sqrt_price_from_input(
        sqrt_price_x96: SqrtPrice,
        liquidity: Liquidity,
        amount_in: U256,
        zero_for_one: bool,
    ) -> Result<SqrtPrice> {
        if sqrt_price_x96.to_u256().is_zero() || liquidity.to_u128() == 0 {
            return Err(MathError::InvalidPrice);
        }

        if zero_for_one {
            Self::get_next_sqrt_price_from_amount0_rounding_up(sqrt_price_x96, liquidity, amount_in, true)
        } else {
            Self::get_next_sqrt_price_from_amount1_rounding_down(sqrt_price_x96, liquidity, amount_in, true)
        }
    }

    /// Gets the next sqrt price given an output amount of token0 or token1
    pub fn get_next_sqrt_price_from_output(
        sqrt_price_x96: SqrtPrice,
        liquidity: Liquidity,
        amount_out: U256,
        zero_for_one: bool,
    ) -> Result<SqrtPrice> {
        if sqrt_price_x96.to_u256().is_zero() || liquidity.to_u128() == 0 {
            return Err(MathError::InvalidPrice);
        }

        if zero_for_one {
            Self::get_next_sqrt_price_from_amount1_rounding_down(sqrt_price_x96, liquidity, amount_out, false)
        } else {
            Self::get_next_sqrt_price_from_amount0_rounding_up(sqrt_price_x96, liquidity, amount_out, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_sqrt_price_from_input() {
        let sqrt_price = SqrtPrice::new(U256::from(1000000));
        let liquidity = Liquidity::new(500000);
        let amount = U256::from(1000);
        
        let result = SqrtPriceMath::get_next_sqrt_price_from_input(
            sqrt_price,
            liquidity,
            amount,
            true,
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_price() {
        let sqrt_price = SqrtPrice::new(U256::zero());
        let liquidity = Liquidity::new(500000);
        let amount = U256::from(1000);
        
        let result = SqrtPriceMath::get_next_sqrt_price_from_input(
            sqrt_price,
            liquidity,
            amount,
            true,
        );
        
        assert!(matches!(result, Err(MathError::InvalidPrice)));
    }
} 