use std::collections::HashMap;
use primitive_types::U256;

use crate::core::{
    math::types::SqrtPrice,
    state::{
        Pool,
        BalanceDelta,
        Result as StateResult,
        StateError,
    },
    hooks::{
        HookRegistry,
        HookFlags,
        hook_interface::PoolKey,
    },
};

/// Manages the lifecycle and operations of pools
pub struct PoolManager {
    /// Registry of hooks
    hook_registry: HookRegistry,
    /// Mapping of pool keys to pools
    pools: HashMap<PoolKey, Pool>,
}

impl PoolManager {
    /// Creates a new pool manager
    pub fn new() -> Self {
        Self {
            hook_registry: HookRegistry::new(),
            pools: HashMap::new(),
        }
    }

    /// Initializes a new pool
    pub fn initialize_pool(
        &mut self,
        key: PoolKey,
        sqrt_price_x96: SqrtPrice,
    ) -> StateResult<i32> {
        // Check if pool already exists
        if self.pools.contains_key(&key) {
            return Err(StateError::PoolAlreadyInitialized);
        }

        // Create and initialize pool
        let mut pool = Pool::new();
        let tick = pool.initialize(sqrt_price_x96, key.fee)?;

        // Add pool to manager
        self.pools.insert(key, pool);

        Ok(tick)
    }

    /// Gets a reference to a pool
    pub fn get_pool(&self, key: &PoolKey) -> Option<&Pool> {
        self.pools.get(key)
    }

    /// Gets a mutable reference to a pool
    pub fn get_pool_mut(&mut self, key: &PoolKey) -> Option<&mut Pool> {
        self.pools.get_mut(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_key() -> PoolKey {
        PoolKey {
            token0: [0u8; 20],
            token1: [1u8; 20],
            fee: 3000, // 0.3%
            tick_spacing: 60,
            hooks: [0u8; 20],
            extension_data: Vec::new(),
        }
    }

    #[test]
    fn test_initialize_pool() {
        let mut manager = PoolManager::new();
        let key = create_test_key();
        let sqrt_price = SqrtPrice::new(U256::from(1 << 96)); // 1.0 price

        let tick = manager.initialize_pool(
            key.clone(),
            sqrt_price,
        ).unwrap();

        assert_eq!(tick, 0);

        // Verify pool was created
        let pool = manager.get_pool(&key).unwrap();
        assert_eq!(pool.slot0.tick, 0);
        assert_eq!(pool.slot0.lp_fee, 3000);
    }
} 