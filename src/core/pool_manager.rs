use std::collections::HashMap;
use primitive_types::U256;
use ethers::types::Address;

use crate::core::{
    math::types::SqrtPrice,
    state::{
        Pool,
        Result as StateResult,
        StateError,
    },
    flash_loan::{
        FlashLoanManager,
        FlashLoanCallback,
        Currency,
        FlashLoanError,
    },
};

/// 简单的池子键
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct PoolKey {
    pub token0: Address,
    pub token1: Address,
    pub fee: u32,
}

/// Manages the lifecycle and operations of pools
pub struct PoolManager {
    /// Mapping of pool keys to pools
    pools: HashMap<PoolKey, Pool>,
    /// Flash loan manager
    flash_loan_manager: FlashLoanManager,
}

impl PoolManager {
    /// Creates a new pool manager
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            flash_loan_manager: FlashLoanManager::new(),
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
    
    /// Unlocks the pool manager to execute a flash loan callback
    pub fn unlock<C: FlashLoanCallback>(&mut self, callback: &mut C, data: &[u8]) -> Result<Vec<u8>, FlashLoanError> {
        self.flash_loan_manager.unlock(callback, data)
    }
    
    /// Take a currency (flash loan)
    pub fn take(&self, currency: Currency, to: Address, amount: u128) -> Result<(), FlashLoanError> {
        self.flash_loan_manager.take(currency, to, amount)
    }
    
    /// Settle an amount of currency (repay flash loan)
    pub fn settle(&mut self, recipient: Address, value: U256) -> Result<U256, FlashLoanError> {
        self.flash_loan_manager.settle(recipient, value)
    }
    
    /// Settle on behalf of a recipient
    pub fn settle_for(&mut self, recipient: Address, value: U256) -> Result<U256, FlashLoanError> {
        self.flash_loan_manager.settle(recipient, value)
    }
    
    /// Sync a currency for settling
    pub fn sync(&mut self, currency: Currency) {
        self.flash_loan_manager.sync(currency)
    }
    
    /// Get the delta for a currency and address
    pub fn get_delta(&self, address: Address, currency: Currency) -> i128 {
        self.flash_loan_manager.get_delta(address, currency)
    }
    
    /// Clear a positive delta (used for dust amounts)
    pub fn clear(&self, currency: Currency, address: Address, amount: u128) -> Result<(), FlashLoanError> {
        self.flash_loan_manager.clear(currency, address, amount)
    }
    
    /// Check if the pool manager is unlocked
    pub fn is_unlocked(&self) -> bool {
        self.flash_loan_manager.lock.is_unlocked()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_key() -> PoolKey {
        PoolKey {
            token0: Address::from_low_u64_be(0),
            token1: Address::from_low_u64_be(1),
            fee: 3000, // 0.3%
        }
    }

    #[test]
    fn test_initialize_pool() {
        let mut manager = PoolManager::new();
        let key = create_test_key();
        let sqrt_price = SqrtPrice::new(U256::from(1u128 << 96)); // 1.0 price

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
    
    // Test for flash loan functionality
    struct TestFlashLoanCallback {
        currency: Currency,
        amount: u128,
        address: Address,
    }
    
    impl FlashLoanCallback for TestFlashLoanCallback {
        fn unlock_callback(&mut self, _data: &[u8]) -> Result<Vec<u8>, FlashLoanError> {
            // In a real scenario, this would involve the pool manager
            // For testing, we simply return success
            println!("Simulating flash loan operations");
            
            Ok(Vec::new())
        }
    }
    
    #[test]
    fn test_flash_loan() {
        let mut manager = PoolManager::new();
        let address = Address::from_low_u64_be(1);
        let currency = Currency::from_address(Address::from_low_u64_be(2));
        let amount = 1000;
        
        let mut callback = TestFlashLoanCallback {
            currency,
            amount,
            address,
        };
        
        // Execute flash loan
        let result = manager.unlock(&mut callback, &[]);
        assert!(result.is_ok());
    }
} 