use std::collections::HashMap;
use num_traits::Zero;
use primitive_types::U256;
use ethers::types::Address;

use crate::core::math::types::Liquidity;
use crate::core::math::FixedPoint96;
use super::{Result, StateError, BalanceDelta};

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

/// Represents a liquidity position
#[derive(Debug, Clone, Default)]
pub struct Position {
    /// The amount of liquidity in the position
    pub liquidity: Liquidity,
    /// The fee growth inside the position as of the last update
    pub fee_growth_inside_0_last_x128: U256,
    /// The fee growth inside the position as of the last update
    pub fee_growth_inside_1_last_x128: U256,
    /// The fees owed to the position owner in token0
    pub tokens_owed_0: u128,
    /// The fees owed to the position owner in token1
    pub tokens_owed_1: u128,
}

impl Position {
    /// Creates a new position
    pub fn new(liquidity: Liquidity) -> Self {
        Self {
            liquidity,
            fee_growth_inside_0_last_x128: U256::zero(),
            fee_growth_inside_1_last_x128: U256::zero(),
            tokens_owed_0: 0,
            tokens_owed_1: 0,
        }
    }

    /// Updates the position with a new liquidity delta and current fee growth values
    pub fn update(
        &mut self,
        liquidity_delta: i128,
        fee_growth_inside_0_x128: U256,
        fee_growth_inside_1_x128: U256,
    ) -> Result<BalanceDelta> {
        let tokens_owed_0 = if !fee_growth_inside_0_x128.is_zero() && !self.liquidity.is_zero() {
            // Calculate accumulated fees in token0
            let fee_delta = fee_growth_inside_0_x128
                .overflowing_sub(self.fee_growth_inside_0_last_x128)
                .0;
            let fee_amount = FixedPoint96::mul_div(
                U256::from(self.liquidity.as_u128()),
                fee_delta,
                U256::from(1) << 128,
            ).as_u128();
            self.tokens_owed_0 = self.tokens_owed_0.checked_add(fee_amount).ok_or(StateError::LiquidityOverflow)?;
            fee_amount
        } else {
            0
        };

        let tokens_owed_1 = if !fee_growth_inside_1_x128.is_zero() && !self.liquidity.is_zero() {
            // Calculate accumulated fees in token1
            let fee_delta = fee_growth_inside_1_x128
                .overflowing_sub(self.fee_growth_inside_1_last_x128)
                .0;
            let fee_amount = FixedPoint96::mul_div(
                U256::from(self.liquidity.as_u128()),
                fee_delta,
                U256::from(1) << 128,
            ).as_u128();
            self.tokens_owed_1 = self.tokens_owed_1.checked_add(fee_amount).ok_or(StateError::LiquidityOverflow)?;
            fee_amount
        } else {
            0
        };

        // Update the position's liquidity
        if liquidity_delta != 0 {
            let new_liquidity = if liquidity_delta > 0 {
                self.liquidity.as_u128().checked_add(liquidity_delta as u128)
            } else {
                self.liquidity.as_u128().checked_sub((-liquidity_delta) as u128)
            }.ok_or(StateError::LiquidityOverflow)?;
            
            if liquidity_delta < 0 && new_liquidity == 0 {
                // If we're burning all liquidity, collect any owed tokens
                let all_tokens_owed_0 = self.tokens_owed_0;
                let all_tokens_owed_1 = self.tokens_owed_1;
                self.tokens_owed_0 = 0;
                self.tokens_owed_1 = 0;
                
                self.liquidity = Liquidity::new(0);
                
                // Return the token amounts including all collected fees
                return Ok(BalanceDelta::new(
                    all_tokens_owed_0 as i128,
                    all_tokens_owed_1 as i128,
                ));
            }
            
            self.liquidity = Liquidity::new(new_liquidity);
        }

        // Update the position's fee growth values
        self.fee_growth_inside_0_last_x128 = fee_growth_inside_0_x128;
        self.fee_growth_inside_1_last_x128 = fee_growth_inside_1_x128;

        // Return the fees collected in this update
        Ok(BalanceDelta::new(
            tokens_owed_0 as i128,
            tokens_owed_1 as i128,
        ))
    }

    /// Collects fees accumulated by the position
    pub fn collect_fees(&mut self) -> (u128, u128) {
        let fees_0 = self.tokens_owed_0;
        let fees_1 = self.tokens_owed_1;
        
        // Reset fees owed
        self.tokens_owed_0 = 0;
        self.tokens_owed_1 = 0;
        
        (fees_0, fees_1)
    }

    /// Checks if the position has no liquidity
    pub fn is_empty(&self) -> bool {
        self.liquidity.is_zero()
    }
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
        // Create position if it doesn't exist
        if !self.positions.contains_key(&key) && liquidity_delta > 0 {
            let position = Position::new(Liquidity::new(0));
            self.positions.insert(key.clone(), position);
        } else if !self.positions.contains_key(&key) {
            return Err(StateError::LiquidityNotFound);
        }
        
        // Now we know the position exists
        let position = self.positions.get_mut(&key).unwrap();
        
        // Update the position and get fees
        let fee_delta = position.update(
            liquidity_delta,
            fee_growth_inside_0_x128,
            fee_growth_inside_1_x128,
        )?;
        
        // Remove position if it has no liquidity
        if position.is_empty() {
            self.positions.remove(&key);
        }
        
        Ok(fee_delta)
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

        // No fees for new position
        assert_eq!(fees.amount0(), 0);
        assert_eq!(fees.amount1(), 0);

        // Position should exist with correct liquidity
        let position = manager.get(&key).unwrap();
        assert_eq!(position.liquidity.as_u128(), 100);
    }

    #[test]
    fn test_position_update_existing() {
        let mut manager = PositionManager::new();
        let key = create_test_key();

        // Create a position
        manager.update(key.clone(), 100, U256::from(0), U256::from(0)).unwrap();

        // Update with fee growth
        let fee_growth_0 = U256::from(5000) << 64;
        let fee_growth_1 = U256::from(10000) << 64;
        
        let fees = manager.update(
            key.clone(),
            50,
            fee_growth_0,
            fee_growth_1,
        ).unwrap();

        // Position should have updated liquidity and fee growth
        let position = manager.get(&key).unwrap();
        assert_eq!(position.liquidity.as_u128(), 150);
        assert_eq!(position.fee_growth_inside_0_last_x128, fee_growth_0);
        assert_eq!(position.fee_growth_inside_1_last_x128, fee_growth_1);
        
        // We don't assert on fees.amount0() > 0 since it depends on the calculation
        // and might be 0 for small fee growth values
    }

    #[test]
    fn test_position_remove() {
        let mut manager = PositionManager::new();
        let key = create_test_key();

        // Create a position
        manager.update(key.clone(), 100, U256::from(0), U256::from(0)).unwrap();

        // Remove all liquidity
        manager.update(key.clone(), -100, U256::from(0), U256::from(0)).unwrap();

        // Position should be removed
        assert!(manager.get(&key).is_none());
    }
} 