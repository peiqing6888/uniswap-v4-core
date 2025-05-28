use primitive_types::U256;
use crate::core::math::{
    MathError,
    Result,
    SqrtPriceMath,
    FullMath,
    types::{SqrtPrice, Liquidity},
};

/// Computes the result of a swap within ticks
pub struct SwapMath;

impl SwapMath {
    /// The swap fee is represented in hundredths of a bip, so the max is 100%
    pub const MAX_SWAP_FEE: u32 = 1_000_000;
    
    /// The denominator for fee calculations
    pub const FEE_DENOMINATOR: u32 = Self::MAX_SWAP_FEE;

    /// Computes the sqrt price target for the next swap step
    /// This function is optimized for performance with inline attribute
    #[inline]
    pub fn get_sqrt_price_target(
        zero_for_one: bool,
        sqrt_price_next_x96: SqrtPrice,
        sqrt_price_limit_x96: SqrtPrice,
    ) -> SqrtPrice {
        let next = sqrt_price_next_x96.to_u256();
        let limit = sqrt_price_limit_x96.to_u256();
        
        // Use a more efficient branching approach
        let result = if zero_for_one {
            next.max(limit)
        } else {
            next.min(limit)
        };
        
        SqrtPrice::new(result)
    }
    
    /// Calculate the fee amount from the given input amount and fee rate
    /// This helper function simplifies fee calculations throughout the code
    #[inline]
    pub fn calculate_fee_amount(amount: U256, fee_pips: u32) -> Result<U256> {
        if fee_pips >= Self::MAX_SWAP_FEE {
            return Err(MathError::InvalidPrice);
        }
        
        if fee_pips == 0 {
            return Ok(U256::zero());
        }
        
        FullMath::mul_div_rounding_up(
            amount,
            U256::from(fee_pips),
            U256::from(Self::MAX_SWAP_FEE - fee_pips),
        ).ok_or(MathError::Overflow)
    }
    
    /// Calculate the amount after applying fees
    /// This helper function simplifies fee calculations throughout the code
    #[inline]
    pub fn apply_fee(amount: U256, fee_pips: u32) -> Result<U256> {
        if fee_pips >= Self::MAX_SWAP_FEE {
            return Err(MathError::InvalidPrice);
        }
        
        if fee_pips == 0 {
            return Ok(amount);
        }
        
        FullMath::mul_div(
            amount,
            U256::from(Self::MAX_SWAP_FEE - fee_pips),
            U256::from(Self::MAX_SWAP_FEE),
        ).ok_or(MathError::Overflow)
    }

    /// Computes the result of swapping some amount in, or amount out, given the parameters of the swap
    /// This optimized version has better error handling and performance characteristics
    #[allow(clippy::too_many_arguments)]
    pub fn compute_swap_step(
        sqrt_price_current_x96: SqrtPrice,
        sqrt_price_target_x96: SqrtPrice,
        liquidity: Liquidity,
        amount_remaining: i128,
        fee_pips: u32,
    ) -> Result<(SqrtPrice, U256, U256, U256)> {
        // Early validation of fee parameter
        if fee_pips > Self::MAX_SWAP_FEE {
            return Err(MathError::InvalidPrice);
        }
        
        // Early validation of liquidity
        if liquidity.to_u128() == 0 {
            return Err(MathError::NotEnoughLiquidity);
        }

        let zero_for_one = sqrt_price_current_x96.to_u256() >= sqrt_price_target_x96.to_u256();
        let exact_in = amount_remaining < 0;

        // Handle exact input swaps
        if exact_in {
            // Convert negative amount to positive for calculations
            let amount_remaining_abs = U256::from((-amount_remaining) as u128);
            
            // Calculate amount after fees
            let amount_remaining_less_fee = Self::apply_fee(amount_remaining_abs, fee_pips)?;

            // Calculate the amount in based on the price target
            let amount_in_target = if zero_for_one {
                SqrtPriceMath::get_amount0_delta(
                    sqrt_price_target_x96,
                    sqrt_price_current_x96,
                    liquidity,
                    true,
                )?
            } else {
                SqrtPriceMath::get_amount1_delta(
                    sqrt_price_current_x96,
                    sqrt_price_target_x96,
                    liquidity,
                    true,
                )?
            };

            // Determine if we can reach the target price with the available input
            if amount_remaining_less_fee >= amount_in_target {
                // We can reach the target price - calculate outputs
                
                // Calculate fee amount
                let fee_amount = if fee_pips == 0 {
                    U256::zero()
                } else {
                    Self::calculate_fee_amount(amount_in_target, fee_pips)?
                };

                // Calculate the corresponding output amount
                let amount_out = if zero_for_one {
                    SqrtPriceMath::get_amount1_delta(
                        sqrt_price_target_x96,
                        sqrt_price_current_x96,
                        liquidity,
                        false,
                    )?
                } else {
                    SqrtPriceMath::get_amount0_delta(
                        sqrt_price_current_x96,
                        sqrt_price_target_x96,
                        liquidity,
                        false,
                    )?
                };

                return Ok((sqrt_price_target_x96, amount_in_target, amount_out, fee_amount));
            } else {
                // We cannot reach the target price - calculate the new price based on input
                let sqrt_price_next_x96 = SqrtPriceMath::get_next_sqrt_price_from_input(
                    sqrt_price_current_x96,
                    liquidity,
                    amount_remaining_less_fee,
                    zero_for_one,
                )?;

                // Calculate the fee amount
                let fee_amount = amount_remaining_abs - amount_remaining_less_fee;

                // Calculate the output amount based on the new price
                let amount_out = if zero_for_one {
                    SqrtPriceMath::get_amount1_delta(
                        sqrt_price_next_x96,
                        sqrt_price_current_x96,
                        liquidity,
                        false,
                    )?
                } else {
                    SqrtPriceMath::get_amount0_delta(
                        sqrt_price_current_x96,
                        sqrt_price_next_x96,
                        liquidity,
                        false,
                    )?
                };

                return Ok((sqrt_price_next_x96, amount_remaining_less_fee, amount_out, fee_amount));
            }
        } else {
            // Handle exact output swaps
            let amount_remaining_abs = U256::from(amount_remaining as u128);
            
            // Calculate the output amount based on the price target
            let amount_out_target = if zero_for_one {
                SqrtPriceMath::get_amount1_delta(
                    sqrt_price_target_x96,
                    sqrt_price_current_x96,
                    liquidity,
                    false,
                )?
            } else {
                SqrtPriceMath::get_amount0_delta(
                    sqrt_price_current_x96,
                    sqrt_price_target_x96,
                    liquidity,
                    false,
                )?
            };

            // Determine if we can reach the target price with the desired output
            if amount_remaining_abs >= amount_out_target {
                // We can reach the target price - calculate inputs
                let amount_in = if zero_for_one {
                    SqrtPriceMath::get_amount0_delta(
                        sqrt_price_target_x96,
                        sqrt_price_current_x96,
                        liquidity,
                        true,
                    )?
                } else {
                    SqrtPriceMath::get_amount1_delta(
                        sqrt_price_current_x96,
                        sqrt_price_target_x96,
                        liquidity,
                        true,
                    )?
                };

                // Calculate the fee amount
                let fee_amount = Self::calculate_fee_amount(amount_in, fee_pips)?;

                return Ok((sqrt_price_target_x96, amount_in, amount_out_target, fee_amount));
            } else {
                // We cannot reach the target price - calculate the new price based on output
                let sqrt_price_next_x96 = SqrtPriceMath::get_next_sqrt_price_from_output(
                    sqrt_price_current_x96,
                    liquidity,
                    amount_remaining_abs,
                    zero_for_one,
                )?;

                // Calculate the input amount based on the new price
                let amount_in = if zero_for_one {
                    SqrtPriceMath::get_amount0_delta(
                        sqrt_price_next_x96,
                        sqrt_price_current_x96,
                        liquidity,
                        true,
                    )?
                } else {
                    SqrtPriceMath::get_amount1_delta(
                        sqrt_price_current_x96,
                        sqrt_price_next_x96,
                        liquidity,
                        true,
                    )?
                };

                // Calculate the fee amount
                let fee_amount = Self::calculate_fee_amount(amount_in, fee_pips)?;

                return Ok((sqrt_price_next_x96, amount_in, amount_remaining_abs, fee_amount));
            }
        }
        
        // This code should never be reached, but we add it to satisfy the compiler
        Err(MathError::InvalidPrice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sqrt_price_target() {
        let next = SqrtPrice::new(U256::from(1000));
        let limit = SqrtPrice::new(U256::from(900));

        // Test zero_for_one = true (should return max)
        let result = SwapMath::get_sqrt_price_target(true, next, limit);
        assert_eq!(result.to_u256(), U256::from(1000));

        // Test zero_for_one = false (should return min)
        let result = SwapMath::get_sqrt_price_target(false, next, limit);
        assert_eq!(result.to_u256(), U256::from(900));
    }
    
    #[test]
    fn test_calculate_fee_amount() {
        // Test with 0.3% fee
        let amount = U256::from(1000);
        let fee_pips = 3000; // 0.3%
        
        let fee = SwapMath::calculate_fee_amount(amount, fee_pips).unwrap();
        assert_eq!(fee, U256::from(3)); // 0.3% of 1000 is 3
        
        // Test with 0% fee
        let fee = SwapMath::calculate_fee_amount(amount, 0).unwrap();
        assert_eq!(fee, U256::zero());
        
        // Test with invalid fee
        let result = SwapMath::calculate_fee_amount(amount, SwapMath::MAX_SWAP_FEE + 1);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_apply_fee() {
        // Test with 0.3% fee
        let amount = U256::from(1000);
        let fee_pips = 3000; // 0.3%
        
        let amount_after_fee = SwapMath::apply_fee(amount, fee_pips).unwrap();
        assert_eq!(amount_after_fee, U256::from(997)); // 1000 - 0.3% = 997
        
        // Test with 0% fee
        let amount_after_fee = SwapMath::apply_fee(amount, 0).unwrap();
        assert_eq!(amount_after_fee, amount);
        
        // Test with invalid fee
        let result = SwapMath::apply_fee(amount, SwapMath::MAX_SWAP_FEE + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_swap_step_exact_in() {
        let current = SqrtPrice::new(U256::from(1000));
        let target = SqrtPrice::new(U256::from(900));
        let liquidity = Liquidity::new(1000);
        let amount_remaining = -100i128;
        let fee_pips = 3000; // 0.3%

        let result = SwapMath::compute_swap_step(
            current,
            target,
            liquidity,
            amount_remaining,
            fee_pips,
        );

        assert!(result.is_ok());
        
        // Test with zero liquidity
        let result = SwapMath::compute_swap_step(
            current,
            target,
            Liquidity::new(0),
            amount_remaining,
            fee_pips,
        );
        
        assert!(matches!(result, Err(MathError::NotEnoughLiquidity)));
    }

    #[test]
    fn test_compute_swap_step_exact_out() {
        let amount_remaining = 100i128;
        let fee_pips = 3000; // 0.3%

        // Using more reasonable price and liquidity values
        let current = SqrtPrice::new(U256::from(1) << 96); // 1.0 price
        let target = SqrtPrice::new((U256::from(1) << 96) * U256::from(95) / U256::from(100)); // 0.95 price
        let liquidity = Liquidity::new(1_000_000); // Larger liquidity value
        
        let result = SwapMath::compute_swap_step(
            current,
            target,
            liquidity,
            amount_remaining,
            fee_pips,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_fee() {
        let current = SqrtPrice::new(U256::from(1000));
        let target = SqrtPrice::new(U256::from(900));
        let liquidity = Liquidity::new(1000);
        let amount_remaining = 100i128;
        let fee_pips = SwapMath::MAX_SWAP_FEE + 1;

        let result = SwapMath::compute_swap_step(
            current,
            target,
            liquidity,
            amount_remaining,
            fee_pips,
        );

        assert!(matches!(result, Err(MathError::InvalidPrice)));
    }
} 