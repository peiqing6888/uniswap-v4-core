use ethers::types::{Address, U256};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Currency type represents a token in the Uniswap ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Currency(pub Address);

// ZERO_ADDRESS is the address used to represent native ETH
pub const ZERO_ADDRESS: Address = Address::zero();

impl Currency {
    /// Returns true if this is the native currency (ETH)
    pub fn is_native(&self) -> bool {
        self.0 == ZERO_ADDRESS
    }

    /// Creates a currency from an address
    pub fn from_address(address: Address) -> Self {
        Self(address)
    }

    /// Get the underlying address
    pub fn address(&self) -> Address {
        self.0
    }

    /// Convert to a uint256 for ERC6909 token ID
    pub fn to_id(&self) -> U256 {
        U256::from_big_endian(&self.0.as_bytes()[..])
    }
    
    /// Convert from a uint256 token ID to a Currency
    pub fn from_id(id: U256) -> Self {
        let mut bytes = [0u8; 32];
        id.to_big_endian(&mut bytes);
        let mut addr_bytes = [0u8; 20];
        addr_bytes.copy_from_slice(&bytes[12..32]);
        Self(Address::from_slice(&addr_bytes))
    }
}

/// Represents a delta (positive or negative) for a specific currency
#[derive(Debug, Clone, Copy)]
pub struct CurrencyDelta {
    pub currency: Currency,
    pub amount: i128,
}

impl CurrencyDelta {
    pub fn new(currency: Currency, amount: i128) -> Self {
        Self { currency, amount }
    }
}

/// Stores the currency deltas for all currencies and accounts
#[derive(Debug, Default)]
pub struct CurrencyDeltaTracker {
    // Maps (address, currency) to delta
    deltas: HashMap<(Address, Currency), i128>,
    // Count of non-zero deltas
    non_zero_delta_count: usize,
}

impl CurrencyDeltaTracker {
    /// Create a new currency delta tracker
    pub fn new() -> Self {
        Self {
            deltas: HashMap::new(),
            non_zero_delta_count: 0,
        }
    }
    
    /// Get the delta for a specific address and currency
    pub fn get_delta(&self, address: Address, currency: Currency) -> i128 {
        *self.deltas.get(&(address, currency)).unwrap_or(&0)
    }
    
    /// Apply a delta for an address and currency
    pub fn apply_delta(&mut self, address: Address, currency: Currency, delta: i128) -> (i128, i128) {
        if delta == 0 {
            return (0, 0);
        }
        
        let key = (address, currency);
        let previous = *self.deltas.get(&key).unwrap_or(&0);
        let next = previous + delta;
        
        // Update non-zero delta count
        if previous == 0 && next != 0 {
            self.non_zero_delta_count += 1;
        } else if previous != 0 && next == 0 {
            self.non_zero_delta_count -= 1;
        }
        
        self.deltas.insert(key, next);
        (previous, next)
    }
    
    /// Get the count of non-zero deltas
    pub fn non_zero_delta_count(&self) -> usize {
        self.non_zero_delta_count
    }
    
    /// Clear all deltas for a specific address
    pub fn clear_deltas_for_address(&mut self, address: Address) {
        let to_remove = self.deltas.keys()
            .filter(|(addr, _)| *addr == address)
            .filter(|(addr, curr)| *self.deltas.get(&(*addr, *curr)).unwrap_or(&0) != 0)
            .map(|k| *k)
            .collect::<Vec<_>>();
            
        self.non_zero_delta_count -= to_remove.len();
        
        for key in to_remove {
            self.deltas.remove(&key);
        }
    }
    
    /// Clear all deltas
    pub fn clear_all_deltas(&mut self) {
        self.deltas.clear();
        self.non_zero_delta_count = 0;
    }
}

/// Thread-safe version of the currency delta tracker
#[derive(Debug, Default, Clone)]
pub struct SharedCurrencyDeltaTracker {
    inner: Arc<RwLock<CurrencyDeltaTracker>>,
}

impl SharedCurrencyDeltaTracker {
    /// Create a new shared currency delta tracker
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(CurrencyDeltaTracker::new())),
        }
    }
    
    /// Get the delta for a specific address and currency
    pub fn get_delta(&self, address: Address, currency: Currency) -> i128 {
        self.inner.read().unwrap().get_delta(address, currency)
    }
    
    /// Apply a delta for an address and currency
    pub fn apply_delta(&self, address: Address, currency: Currency, delta: i128) -> (i128, i128) {
        self.inner.write().unwrap().apply_delta(address, currency, delta)
    }
    
    /// Get the count of non-zero deltas
    pub fn non_zero_delta_count(&self) -> usize {
        self.inner.read().unwrap().non_zero_delta_count()
    }
    
    /// Clear all deltas for a specific address
    pub fn clear_deltas_for_address(&self, address: Address) {
        self.inner.write().unwrap().clear_deltas_for_address(address)
    }
    
    /// Clear all deltas
    pub fn clear_all_deltas(&self) {
        self.inner.write().unwrap().clear_all_deltas()
    }
} 