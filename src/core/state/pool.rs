use primitive_types::U256;

use crate::core::math::{
    TickMath,
    SqrtPriceMath,
    SwapMath,
    types::{SqrtPrice, Liquidity},
    Result as MathResult,
};

use super::{
    Result,
    StateError,
    types::{Slot0, BalanceDelta},
    tick::TickManager,
    position::{PositionManager, PositionKey},
};

/// Pool state and operations
pub struct Pool {
    /// The most frequently accessed state
    pub slot0: Slot0,
    /// The current protocol fee growth of token0 accumulated per unit of liquidity
    pub fee_growth_global_0_x128: U256,
    /// The current protocol fee growth of token1 accumulated per unit of liquidity
    pub fee_growth_global_1_x128: U256,
    /// The current liquidity in the pool
    pub liquidity: Liquidity,
    /// The tick manager
    pub tick_manager: TickManager,
    /// The position manager
    pub position_manager: PositionManager,
}

impl Pool {
    /// Creates a new pool
    pub fn new() -> Self {
        Self {
            slot0: Slot0 {
                sqrt_price_x96: SqrtPrice::new(U256::zero()),
                tick: 0,
                protocol_fee: 0,
                lp_fee: 0,
            },
            fee_growth_global_0_x128: U256::zero(),
            fee_growth_global_1_x128: U256::zero(),
            liquidity: Liquidity::new(0),
            tick_manager: TickManager::new(),
            position_manager: PositionManager::new(),
        }
    }

    /// Initializes the pool with an initial sqrt price and LP fee
    pub fn initialize(
        &mut self,
        sqrt_price_x96: SqrtPrice,
        lp_fee: u32,
    ) -> Result<i32> {
        if !self.slot0.sqrt_price_x96.is_zero() {
            return Err(StateError::PoolAlreadyInitialized);
        }

        let tick = TickMath::get_tick_at_sqrt_price(sqrt_price_x96)
            .map_err(|_| StateError::InvalidPrice)?;

        self.slot0 = Slot0 {
            sqrt_price_x96,
            tick,
            protocol_fee: 0,
            lp_fee,
        };

        Ok(tick)
    }

    /// Sets the protocol fee
    pub fn set_protocol_fee(&mut self, protocol_fee: u32) -> Result<()> {
        if self.slot0.sqrt_price_x96.is_zero() {
            return Err(StateError::PoolNotInitialized);
        }
        self.slot0.protocol_fee = protocol_fee;
        Ok(())
    }

    /// Sets the LP fee
    pub fn set_lp_fee(&mut self, lp_fee: u32) -> Result<()> {
        if self.slot0.sqrt_price_x96.is_zero() {
            return Err(StateError::PoolNotInitialized);
        }
        self.slot0.lp_fee = lp_fee;
        Ok(())
    }

    /// Modifies the position's liquidity and returns the resulting balance changes
    pub fn modify_position(
        &mut self,
        owner: [u8; 20],
        tick_lower: i32,
        tick_upper: i32,
        liquidity_delta: i128,
        tick_spacing: i32,
        salt: [u8; 32],
    ) -> Result<(BalanceDelta, BalanceDelta)> {
        if tick_lower >= tick_upper {
            return Err(StateError::TicksMisordered(tick_lower, tick_upper));
        }
        if tick_lower < TickMath::MIN_TICK {
            return Err(StateError::TickLowerOutOfBounds(tick_lower));
        }
        if tick_upper > TickMath::MAX_TICK {
            return Err(StateError::TickUpperOutOfBounds(tick_upper));
        }

        let mut balance_delta = BalanceDelta::default();
        let mut fee_delta = BalanceDelta::default();

        // Update the ticks and check liquidity bounds
        if liquidity_delta != 0 {
            let (flipped_lower, liquidity_gross_after_lower) = self.tick_manager.update_tick(
                tick_lower,
                liquidity_delta,
                self.fee_growth_global_0_x128,
                self.fee_growth_global_1_x128,
                false,
                &self.slot0,
            )?;

            let (flipped_upper, liquidity_gross_after_upper) = self.tick_manager.update_tick(
                tick_upper,
                liquidity_delta,
                self.fee_growth_global_0_x128,
                self.fee_growth_global_1_x128,
                true,
                &self.slot0,
            )?;

            if liquidity_delta > 0 {
                let max_liquidity_per_tick = Self::tick_spacing_to_max_liquidity_per_tick(tick_spacing);
                if liquidity_gross_after_lower > max_liquidity_per_tick {
                    return Err(StateError::TickLiquidityOverflow(tick_lower));
                }
                if liquidity_gross_after_upper > max_liquidity_per_tick {
                    return Err(StateError::TickLiquidityOverflow(tick_upper));
                }
            }

            // Update the position
            let key = PositionKey {
                owner,
                tick_lower,
                tick_upper,
                salt,
            };

            let (fee_growth_inside_0_x128, fee_growth_inside_1_x128) = self.tick_manager
                .get_fee_growth_inside(
                    tick_lower,
                    tick_upper,
                    self.slot0.tick,
                    self.fee_growth_global_0_x128,
                    self.fee_growth_global_1_x128,
                );

            fee_delta = self.position_manager.update(
                key,
                liquidity_delta,
                fee_growth_inside_0_x128,
                fee_growth_inside_1_x128,
            )?;

            // Update pool liquidity if we're in range
            if self.slot0.tick >= tick_lower && self.slot0.tick < tick_upper {
                let liquidity_next = if liquidity_delta > 0 {
                    self.liquidity.as_u128().checked_add(liquidity_delta as u128)
                } else {
                    self.liquidity.as_u128().checked_sub((-liquidity_delta) as u128)
                }.ok_or(StateError::TickLiquidityOverflow(0))?;

                self.liquidity = Liquidity::new(liquidity_next);
            }

            // Calculate token amounts from liquidity change
            if liquidity_delta != 0 {
                let (amount0, amount1) = if self.slot0.tick < tick_lower {
                    // Current tick below position
                    let price_lower = TickMath::get_sqrt_price_at_tick(tick_lower)
                        .map_err(|_| StateError::InvalidPrice)?;
                    let price_upper = TickMath::get_sqrt_price_at_tick(tick_upper)
                        .map_err(|_| StateError::InvalidPrice)?;
                    (
                        SqrtPriceMath::get_amount0_delta(
                            price_lower,
                            price_upper,
                            liquidity_delta.abs() as u128,
                            true,
                        ).map_err(|_| StateError::InvalidPrice)?,
                        U256::zero(),
                    )
                } else if self.slot0.tick < tick_upper {
                    // Current tick inside position
                    let price_current = self.slot0.sqrt_price_x96;
                    let price_upper = TickMath::get_sqrt_price_at_tick(tick_upper)
                        .map_err(|_| StateError::InvalidPrice)?;
                    (
                        SqrtPriceMath::get_amount0_delta(
                            price_current,
                            price_upper,
                            liquidity_delta.abs() as u128,
                            true,
                        ).map_err(|_| StateError::InvalidPrice)?,
                        SqrtPriceMath::get_amount1_delta(
                            price_current,
                            price_upper,
                            liquidity_delta.abs() as u128,
                            true,
                        ).map_err(|_| StateError::InvalidPrice)?,
                    )
                } else {
                    // Current tick above position
                    let price_lower = TickMath::get_sqrt_price_at_tick(tick_lower)
                        .map_err(|_| StateError::InvalidPrice)?;
                    let price_upper = TickMath::get_sqrt_price_at_tick(tick_upper)
                        .map_err(|_| StateError::InvalidPrice)?;
                    (
                        U256::zero(),
                        SqrtPriceMath::get_amount1_delta(
                            price_lower,
                            price_upper,
                            liquidity_delta.abs() as u128,
                            true,
                        ).map_err(|_| StateError::InvalidPrice)?,
                    )
                };

                balance_delta = BalanceDelta::new(
                    if liquidity_delta > 0 {
                        -(amount0.try_into().unwrap_or(i128::MAX))
                    } else {
                        amount0.try_into().unwrap_or(i128::MAX)
                    },
                    if liquidity_delta > 0 {
                        -(amount1.try_into().unwrap_or(i128::MAX))
                    } else {
                        amount1.try_into().unwrap_or(i128::MAX)
                    },
                );
            }
        }

        Ok((balance_delta, fee_delta))
    }

    /// Calculates the maximum liquidity per tick at the given tick spacing
    fn tick_spacing_to_max_liquidity_per_tick(tick_spacing: i32) -> u128 {
        let min_tick = (TickMath::MIN_TICK / tick_spacing) * tick_spacing;
        let max_tick = (TickMath::MAX_TICK / tick_spacing) * tick_spacing;
        let num_ticks = ((max_tick - min_tick) / tick_spacing + 1) as u128;
        u128::MAX / num_ticks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_initialization() {
        let mut pool = Pool::new();
        let sqrt_price = SqrtPrice::new(U256::from(1 << 96));
        let lp_fee = 3000; // 0.3%

        let tick = pool.initialize(sqrt_price, lp_fee).unwrap();
        assert_eq!(tick, 0);
        assert_eq!(pool.slot0.lp_fee, 3000);
    }

    #[test]
    fn test_modify_position() {
        let mut pool = Pool::new();
        let sqrt_price = SqrtPrice::new(U256::from(1 << 96));
        pool.initialize(sqrt_price, 3000).unwrap();

        let owner = [0u8; 20];
        let salt = [0u8; 32];
        let tick_spacing = 60;

        // Add liquidity
        let (balance_delta, fee_delta) = pool.modify_position(
            owner,
            -120,
            120,
            1000,
            tick_spacing,
            salt,
        ).unwrap();

        // Check that tokens were taken from the user
        assert!(balance_delta.amount0 < 0);
        assert!(balance_delta.amount1 < 0);
        // No fees for first position
        assert_eq!(fee_delta.amount0, 0);
        assert_eq!(fee_delta.amount1, 0);

        // Remove liquidity
        let (balance_delta, _) = pool.modify_position(
            owner,
            -120,
            120,
            -1000,
            tick_spacing,
            salt,
        ).unwrap();

        // Check that tokens were returned to the user
        assert!(balance_delta.amount0 > 0);
        assert!(balance_delta.amount1 > 0);
    }
} 