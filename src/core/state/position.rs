use std::collections::HashMap;
use std::collections::BTreeMap;
use num_traits::Zero;
use primitive_types::U256;

use crate::core::math::types::{Liquidity, SqrtPrice};
use super::{Result, StateError, types::{Position, BalanceDelta}};

/// Key for identifying a position
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PositionKey {
    /// The owner of the position
    pub owner: [u8; 20],
    /// The lower tick boundary
    pub tick_lower: i32,
    /// The upper tick boundary
    pub tick_upper: i32,
    /// Additional salt to distinguish positions with same parameters
    pub salt: [u8; 32],
}

/// Manages positions in a pool
pub struct PositionManager {
    /// Mapping of position key to position state
    positions: HashMap<PositionKey, Position>,
}

impl PositionManager {
    /// Creates a new position manager
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

    /// Gets a position by its key
    pub fn get(&self, key: &PositionKey) -> Option<&Position> {
        self.positions.get(key)
    }

    /// Gets a mutable reference to a position by its key
    pub fn get_mut(&mut self, key: &PositionKey) -> Option<&mut Position> {
        self.positions.get_mut(key)
    }

    /// Updates a position with the given liquidity delta and returns the fees owed
    pub fn update(
        &mut self,
        key: PositionKey,
        liquidity_delta: i128,
        fee_growth_inside_0_x128: U256,
        fee_growth_inside_1_x128: U256,
    ) -> Result<BalanceDelta> {
        let position = self.positions.entry(key.clone()).or_default();

        let fees_owed = if position.liquidity.is_zero() {
            // For a new position, no fees are owed
            BalanceDelta::default()
        } else {
            // Calculate fees owed
            let fee_delta_0 = fee_growth_inside_0_x128
                .saturating_sub(position.fee_growth_inside_0_last_x128)
                .checked_mul(U256::from(position.liquidity.as_u128()))
                .ok_or(StateError::TickLiquidityOverflow(0))?
                >> 128;

            let fee_delta_1 = fee_growth_inside_1_x128
                .saturating_sub(position.fee_growth_inside_1_last_x128)
                .checked_mul(U256::from(position.liquidity.as_u128()))
                .ok_or(StateError::TickLiquidityOverflow(0))?
                >> 128;

            BalanceDelta::new(
                fee_delta_0.try_into().unwrap_or(i128::MAX),
                fee_delta_1.try_into().unwrap_or(i128::MAX),
            )
        };

        // Update position state
        if liquidity_delta != 0 {
            let liquidity_next = if liquidity_delta > 0 {
                position.liquidity.as_u128().checked_add(liquidity_delta as u128)
            } else {
                position.liquidity.as_u128().checked_sub((-liquidity_delta) as u128)
            }.ok_or(StateError::TickLiquidityOverflow(0))?;

            position.liquidity = Liquidity::new(liquidity_next);
        }

        position.fee_growth_inside_0_last_x128 = fee_growth_inside_0_x128;
        position.fee_growth_inside_1_last_x128 = fee_growth_inside_1_x128;

        // Remove the position if it has no liquidity
        if position.liquidity.is_zero() {
            self.positions.remove(&key);
        }

        Ok(fees_owed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_key() -> PositionKey {
        PositionKey {
            owner: [0; 20],
            tick_lower: -100,
            tick_upper: 100,
            salt: [0; 32],
        }
    }

    #[test]
    fn test_position_update_new() {
        let mut manager = PositionManager::new();
        let key = create_test_key();

        // Test creating a new position
        let fees = manager.update(
            key.clone(),
            100,
            U256::from(0),
            U256::from(0),
        ).unwrap();

        assert_eq!(fees.amount0, 0);
        assert_eq!(fees.amount1, 0);

        let position = manager.get(&key).unwrap();
        assert_eq!(position.liquidity.as_u128(), 100);
    }

    #[test]
    fn test_position_update_existing() {
        let mut manager = PositionManager::new();
        let key = create_test_key();

        // Create initial position
        manager.update(
            key.clone(),
            100,
            U256::from(0),
            U256::from(0),
        ).unwrap();

        // Update position and accumulate fees
        let fees = manager.update(
            key.clone(),
            50,
            U256::from(100),
            U256::from(200),
        ).unwrap();

        // Check fees calculation
        assert!(fees.amount0 > 0);
        assert!(fees.amount1 > 0);

        let position = manager.get(&key).unwrap();
        assert_eq!(position.liquidity.as_u128(), 150);
    }

    #[test]
    fn test_position_remove() {
        let mut manager = PositionManager::new();
        let key = create_test_key();

        // Create and then remove position
        manager.update(
            key.clone(),
            100,
            U256::from(0),
            U256::from(0),
        ).unwrap();

        manager.update(
            key.clone(),
            -100,
            U256::from(0),
            U256::from(0),
        ).unwrap();

        assert!(manager.get(&key).is_none());
    }
} 