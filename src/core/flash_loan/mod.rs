use primitive_types::U256;
use ethers::types::Address;

pub mod currency;
pub mod lock;
pub mod callback;
pub mod error;
pub mod examples;

pub use currency::*;
pub use lock::*;
pub use callback::*;
pub use error::*;
pub use examples::*;

// Constants
pub const ZERO_ADDRESS: Address = Address::zero();

// Main flash loan module
// This module provides the implementation of flash loans for Uniswap V4

/// Flash loan manager for the Uniswap V4 core
pub struct FlashLoanManager {
    /// Currency delta tracker
    currency_delta_tracker: SharedCurrencyDeltaTracker,
    /// Lock
    pub lock: Lock,
    /// Currency reserves (for settling)
    currency_reserves: CurrencyReserves,
}

/// Currency reserves for settling
#[derive(Debug, Default, Clone)]
pub struct CurrencyReserves {
    /// Currently synced currency (for settling)
    synced_currency: Option<Currency>,
    /// Current reserves of the synced currency
    reserves: U256,
}

impl CurrencyReserves {
    /// Create new currency reserves
    pub fn new() -> Self {
        Self {
            synced_currency: None,
            reserves: U256::zero(),
        }
    }
    
    /// Reset the currency
    pub fn reset_currency(&mut self) {
        self.synced_currency = None;
        self.reserves = U256::zero();
    }
    
    /// Sync the currency and reserves
    pub fn sync_currency_and_reserves(&mut self, currency: Currency, reserves: U256) {
        self.synced_currency = Some(currency);
        self.reserves = reserves;
    }
    
    /// Get the synced currency
    pub fn get_synced_currency(&self) -> Option<Currency> {
        self.synced_currency
    }
    
    /// Get the synced reserves
    pub fn get_synced_reserves(&self) -> U256 {
        self.reserves
    }
}

impl FlashLoanManager {
    /// Create a new flash loan manager
    pub fn new() -> Self {
        Self {
            currency_delta_tracker: SharedCurrencyDeltaTracker::new(),
            lock: Lock::new(),
            currency_reserves: CurrencyReserves::new(),
        }
    }
    
    /// Unlock the pool manager to execute a flash loan callback
    pub fn unlock<C: FlashLoanCallback>(&mut self, callback: &mut C, data: &[u8]) -> Result<Vec<u8>, FlashLoanError> {
        // Create unlock guard (automatically locks on drop)
        let _guard = match UnlockGuard::new(&self.lock) {
            Ok(guard) => guard,
            Err(_) => return Err(FlashLoanError::AlreadyUnlocked),
        };
        
        // Execute the callback
        let result = callback.unlock_callback(data)?;
        
        // Check if all currencies are settled
        if self.currency_delta_tracker.non_zero_delta_count() != 0 {
            return Err(FlashLoanError::CurrencyNotSettled);
        }
        
        Ok(result)
    }
    
    /// Take a currency from the pool manager (flash loan)
    pub fn take(&self, currency: Currency, to: Address, amount: u128) -> Result<(), FlashLoanError> {
        // Check if the pool manager is locked
        if !self.lock.is_unlocked() {
            return Err(FlashLoanError::ManagerLocked);
        }
        
        // Account the delta (negative because it's being taken)
        self.currency_delta_tracker.apply_delta(to, currency, -(amount as i128));
        
        // Note: In a real implementation, you would transfer tokens here
        // This is simplified and would need integration with actual token contracts
        
        Ok(())
    }
    
    /// Settle a currency to the pool manager (repay flash loan)
    pub fn settle(&mut self, recipient: Address, value: U256) -> Result<U256, FlashLoanError> {
        // Check if the pool manager is locked
        if !self.lock.is_unlocked() {
            return Err(FlashLoanError::ManagerLocked);
        }
        
        let currency = match self.currency_reserves.get_synced_currency() {
            Some(curr) => curr,
            None => Currency::from_address(ZERO_ADDRESS), // Default to native currency
        };
        
        let paid: U256 = if currency.is_native() {
            // For native currency, value is the amount being paid
            value
        } else {
            // For ERC20, calculate from the reserves
            if !value.is_zero() {
                return Err(FlashLoanError::NonzeroNativeValue);
            }
            
            // Reserves are guaranteed to be set because currency and reserves are always set together
            let reserves_before = self.currency_reserves.get_synced_reserves();
            
            // In a real implementation, you would get the current balance here
            // This is simplified
            let reserves_now = U256::zero();
            
            // Reset the currency after settling
            self.currency_reserves.reset_currency();
            
            reserves_now.saturating_sub(reserves_before)
        };
        
        // Account the delta (positive because it's being settled)
        self.currency_delta_tracker.apply_delta(
            recipient, 
            currency, 
            paid.low_u128() as i128
        );
        
        Ok(paid)
    }
    
    /// Sync a currency (for settling)
    pub fn sync(&mut self, currency: Currency) {
        // In a real implementation, you would get the current balance here
        // and store it in the reserves
        if currency.is_native() {
            self.currency_reserves.reset_currency();
        } else {
            // Example: we're assuming a balance of 1000 for this example
            let balance = U256::from(1000u64);
            self.currency_reserves.sync_currency_and_reserves(currency, balance);
        }
    }
    
    /// Get the current delta for a currency and address
    pub fn get_delta(&self, address: Address, currency: Currency) -> i128 {
        self.currency_delta_tracker.get_delta(address, currency)
    }
    
    /// Clear a positive delta (used for dust amounts)
    pub fn clear(&self, currency: Currency, address: Address, amount: u128) -> Result<(), FlashLoanError> {
        // Check if the pool manager is locked
        if !self.lock.is_unlocked() {
            return Err(FlashLoanError::ManagerLocked);
        }
        
        let current = self.get_delta(address, currency);
        let amount_delta = amount as i128;
        
        // Must clear an exact positive delta
        if amount_delta != current {
            return Err(FlashLoanError::MustClearExactPositiveDelta);
        }
        
        // Clear the delta by applying a negative delta of the same amount
        self.currency_delta_tracker.apply_delta(address, currency, -amount_delta);
        
        Ok(())
    }
} 