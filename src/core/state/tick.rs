use std::collections::BTreeMap;
use primitive_types::U256;

use crate::core::math::{TickMath, Result as MathResult};
use super::{Result, StateError, types::{TickInfo, Slot0}};

/// Manages the state and operations of ticks in a pool
pub struct TickManager {
    /// Mapping of tick index to tick info
    ticks: BTreeMap<i32, TickInfo>,
    /// Bitmap of initialized ticks
    tick_bitmap: BTreeMap<i16, u256>,
}

impl TickManager {
    /// Creates a new tick manager
    pub fn new() -> Self {
        Self {
            ticks: BTreeMap::new(),
            tick_bitmap: BTreeMap::new(),
        }
    }

    /// Updates a tick's state and returns whether the tick was flipped (initialized or cleared)
    pub fn update_tick(
        &mut self,
        tick: i32,
        liquidity_delta: i128,
        fee_growth_global_0_x128: U256,
        fee_growth_global_1_x128: U256,
        upper: bool,
        slot0: &Slot0,
    ) -> Result<(bool, u128)> {
        let tick_info = self.ticks.entry(tick).or_default();
        let liquidity_gross_before = tick_info.liquidity_gross.as_u128();

        let liquidity_gross_after = if liquidity_delta < 0 {
            // If we're decreasing liquidity, check for underflow
            let decrease = (-liquidity_delta) as u128;
            if liquidity_gross_before < decrease {
                return Err(StateError::TickLiquidityOverflow(tick));
            }
            liquidity_gross_before - decrease
        } else {
            // If we're increasing liquidity, check for overflow
            liquidity_gross_before.checked_add(liquidity_delta as u128)
                .ok_or(StateError::TickLiquidityOverflow(tick))?
        };

        let flipped = (liquidity_gross_after == 0) != (liquidity_gross_before == 0);

        if flipped {
            if liquidity_gross_after == 0 {
                // Remove the tick if it has no liquidity
                self.ticks.remove(&tick);
            } else {
                // Initialize the tick
                tick_info.liquidity_gross = liquidity_gross_after.into();
                tick_info.liquidity_net = liquidity_delta;
                
                // When the tick is initialized, set the fee growth outside to the current global fee growth
                if tick <= slot0.tick {
                    tick_info.fee_growth_outside_0_x128 = fee_growth_global_0_x128;
                    tick_info.fee_growth_outside_1_x128 = fee_growth_global_1_x128;
                }
            }
        } else {
            // Update the tick's liquidity
            tick_info.liquidity_gross = liquidity_gross_after.into();
            tick_info.liquidity_net = tick_info.liquidity_net.checked_add(liquidity_delta)
                .ok_or(StateError::TickLiquidityOverflow(tick))?;
        }

        Ok((flipped, liquidity_gross_after))
    }

    /// Clears a tick's state
    pub fn clear_tick(&mut self, tick: i32) {
        self.ticks.remove(&tick);
    }

    /// Transitions to the next initialized tick
    pub fn next_initialized_tick_within_one_word(
        &self,
        tick: i32,
        tick_spacing: i32,
        lte: bool,
    ) -> MathResult<(i32, bool)> {
        let compressed = tick / tick_spacing;
        if tick < 0 && tick % tick_spacing != 0 {
            compressed -= 1;
        }

        let word_pos = (compressed >> 8) as i16;
        let minimum_tick = TickMath::MIN_TICK / tick_spacing * tick_spacing;
        let maximum_tick = TickMath::MAX_TICK / tick_spacing * tick_spacing;

        if lte {
            let mask = self.tick_bitmap.get(&word_pos).copied().unwrap_or(0);
            let current = (compressed % 256) as u8;
            let masked = mask & ((1u256 << current as u32) - 1);

            if masked != 0 {
                // Find rightmost set bit
                let rightmost = masked.trailing_zeros() as i32;
                let next = (word_pos as i32) * 256 + rightmost;
                let next_tick = next * tick_spacing;
                if next_tick <= maximum_tick {
                    return Ok((next_tick, true));
                }
            }
        } else {
            let mask = self.tick_bitmap.get(&word_pos).copied().unwrap_or(0);
            let current = (compressed % 256) as u8;
            let masked = mask & !((1u256 << current as u32) - 1);

            if masked != 0 {
                // Find leftmost set bit
                let leftmost = 255 - masked.leading_zeros() as i32;
                let next = (word_pos as i32) * 256 + leftmost;
                let next_tick = next * tick_spacing;
                if next_tick >= minimum_tick {
                    return Ok((next_tick, true));
                }
            }
        }

        Ok((if lte { minimum_tick } else { maximum_tick }, false))
    }

    /// Gets the fee growth inside a tick range
    pub fn get_fee_growth_inside(
        &self,
        tick_lower: i32,
        tick_upper: i32,
        tick_current: i32,
        fee_growth_global_0_x128: U256,
        fee_growth_global_1_x128: U256,
    ) -> (U256, U256) {
        let lower = self.ticks.get(&tick_lower).cloned().unwrap_or_default();
        let upper = self.ticks.get(&tick_upper).cloned().unwrap_or_default();

        let fee_growth_below_0_x128;
        let fee_growth_below_1_x128;
        if tick_current >= tick_lower {
            fee_growth_below_0_x128 = lower.fee_growth_outside_0_x128;
            fee_growth_below_1_x128 = lower.fee_growth_outside_1_x128;
        } else {
            fee_growth_below_0_x128 = fee_growth_global_0_x128.saturating_sub(lower.fee_growth_outside_0_x128);
            fee_growth_below_1_x128 = fee_growth_global_1_x128.saturating_sub(lower.fee_growth_outside_1_x128);
        }

        let fee_growth_above_0_x128;
        let fee_growth_above_1_x128;
        if tick_current < tick_upper {
            fee_growth_above_0_x128 = upper.fee_growth_outside_0_x128;
            fee_growth_above_1_x128 = upper.fee_growth_outside_1_x128;
        } else {
            fee_growth_above_0_x128 = fee_growth_global_0_x128.saturating_sub(upper.fee_growth_outside_0_x128);
            fee_growth_above_1_x128 = fee_growth_global_1_x128.saturating_sub(upper.fee_growth_outside_1_x128);
        }

        (
            fee_growth_global_0_x128.saturating_sub(fee_growth_below_0_x128).saturating_sub(fee_growth_above_0_x128),
            fee_growth_global_1_x128.saturating_sub(fee_growth_below_1_x128).saturating_sub(fee_growth_above_1_x128),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::math::types::SqrtPrice;

    #[test]
    fn test_update_tick() {
        let mut manager = TickManager::new();
        let slot0 = Slot0 {
            sqrt_price_x96: SqrtPrice::new(U256::from(1)),
            tick: 0,
            protocol_fee: 0,
            lp_fee: 0,
        };

        // Test initializing a tick
        let (flipped, liquidity) = manager.update_tick(
            1,
            100,
            U256::zero(),
            U256::zero(),
            false,
            &slot0,
        ).unwrap();
        assert!(flipped);
        assert_eq!(liquidity, 100);

        // Test updating an existing tick
        let (flipped, liquidity) = manager.update_tick(
            1,
            50,
            U256::zero(),
            U256::zero(),
            false,
            &slot0,
        ).unwrap();
        assert!(!flipped);
        assert_eq!(liquidity, 150);

        // Test removing a tick
        let (flipped, liquidity) = manager.update_tick(
            1,
            -150,
            U256::zero(),
            U256::zero(),
            false,
            &slot0,
        ).unwrap();
        assert!(flipped);
        assert_eq!(liquidity, 0);
    }

    #[test]
    fn test_fee_growth_inside() {
        let mut manager = TickManager::new();
        let slot0 = Slot0 {
            sqrt_price_x96: SqrtPrice::new(U256::from(1)),
            tick: 0,
            protocol_fee: 0,
            lp_fee: 0,
        };

        // Initialize ticks
        manager.update_tick(
            -100,
            100,
            U256::from(10),
            U256::from(20),
            false,
            &slot0,
        ).unwrap();

        manager.update_tick(
            100,
            100,
            U256::from(30),
            U256::from(40),
            true,
            &slot0,
        ).unwrap();

        // Test fee growth inside the range
        let (fee0, fee1) = manager.get_fee_growth_inside(
            -100,
            100,
            0,
            U256::from(50),
            U256::from(60),
        );

        assert_eq!(fee0, U256::from(40));
        assert_eq!(fee1, U256::from(40));
    }
} 