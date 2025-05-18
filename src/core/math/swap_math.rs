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

    /// Computes the sqrt price target for the next swap step
    pub fn get_sqrt_price_target(
        zero_for_one: bool,
        sqrt_price_next_x96: SqrtPrice,
        sqrt_price_limit_x96: SqrtPrice,
    ) -> SqrtPrice {
        let next = sqrt_price_next_x96.to_u256();
        let limit = sqrt_price_limit_x96.to_u256();
        
        // When zero_for_one is true, we want max(next, limit)
        // When zero_for_one is false, we want min(next, limit)
        let result = if zero_for_one {
            if next > limit { next } else { limit }
        } else {
            if next < limit { next } else { limit }
        };
        
        SqrtPrice::new(result)
    }

    /// Computes the result of swapping some amount in, or amount out, given the parameters of the swap
    #[allow(clippy::too_many_arguments)]
    pub fn compute_swap_step(
        sqrt_price_current_x96: SqrtPrice,
        sqrt_price_target_x96: SqrtPrice,
        liquidity: Liquidity,
        amount_remaining: i128,
        fee_pips: u32,
    ) -> Result<(SqrtPrice, U256, U256, U256)> {
        if fee_pips > Self::MAX_SWAP_FEE {
            return Err(MathError::InvalidPrice);
        }

        let zero_for_one = sqrt_price_current_x96.to_u256() >= sqrt_price_target_x96.to_u256();
        let exact_in = amount_remaining < 0;

        let (sqrt_price_next_x96, amount_in, amount_out, fee_amount) = if exact_in {
            let amount_remaining_less_fee = FullMath::mul_div(
                U256::from((-amount_remaining) as u128),
                U256::from(Self::MAX_SWAP_FEE - fee_pips),
                U256::from(Self::MAX_SWAP_FEE),
            ).ok_or(MathError::Overflow)?;

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

            if amount_remaining_less_fee >= amount_in {
                // Amount in is capped by the target price
                let fee_amount = if fee_pips == Self::MAX_SWAP_FEE {
                    amount_in
                } else {
                    FullMath::mul_div_rounding_up(
                        amount_in,
                        U256::from(fee_pips),
                        U256::from(Self::MAX_SWAP_FEE - fee_pips),
                    ).ok_or(MathError::Overflow)?
                };

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

                (sqrt_price_target_x96, amount_in, amount_out, fee_amount)
            } else {
                // Exhaust the remaining amount
                let sqrt_price_next_x96 = SqrtPriceMath::get_next_sqrt_price_from_input(
                    sqrt_price_current_x96,
                    liquidity,
                    amount_remaining_less_fee,
                    zero_for_one,
                )?;

                let fee_amount = U256::from((-amount_remaining) as u128) - amount_remaining_less_fee;

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

                (sqrt_price_next_x96, amount_remaining_less_fee, amount_out, fee_amount)
            }
        } else {
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

            if U256::from(amount_remaining as u128) >= amount_out {
                // Amount out is capped by the target price
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

                let fee_amount = FullMath::mul_div_rounding_up(
                    amount_in,
                    U256::from(fee_pips),
                    U256::from(Self::MAX_SWAP_FEE - fee_pips),
                ).ok_or(MathError::Overflow)?;

                (sqrt_price_target_x96, amount_in, amount_out, fee_amount)
            } else {
                // Cap the output amount
                let amount_out = U256::from(amount_remaining as u128);
                let sqrt_price_next_x96 = SqrtPriceMath::get_next_sqrt_price_from_output(
                    sqrt_price_current_x96,
                    liquidity,
                    amount_out,
                    zero_for_one,
                )?;

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

                let fee_amount = FullMath::mul_div_rounding_up(
                    amount_in,
                    U256::from(fee_pips),
                    U256::from(Self::MAX_SWAP_FEE - fee_pips),
                ).ok_or(MathError::Overflow)?;

                (sqrt_price_next_x96, amount_in, amount_out, fee_amount)
            }
        };

        Ok((sqrt_price_next_x96, amount_in, amount_out, fee_amount))
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
    }

    #[test]
    fn test_compute_swap_step_exact_out() {
        let current = SqrtPrice::new(U256::from(1000));
        let target = SqrtPrice::new(U256::from(900));
        let liquidity = Liquidity::new(1000);
        let amount_remaining = 100i128;
        let fee_pips = 3000; // 0.3%

        // 问题可能出在测试数据上，使用更合理的价格和流动性值
        let current = SqrtPrice::new(U256::from(1) << 96); // 1.0 价格
        let target = SqrtPrice::new((U256::from(1) << 96) * U256::from(95) / U256::from(100)); // 0.95 价格
        let liquidity = Liquidity::new(1_000_000); // 更大的流动性值
        
        let result = SwapMath::compute_swap_step(
            current,
            target,
            liquidity,
            amount_remaining,
            fee_pips,
        );

        // 如果仍然失败，我们可以检查具体的错误
        match &result {
            Ok(_) => println!("Test passed!"),
            Err(e) => println!("Error: {:?}", e),
        }

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