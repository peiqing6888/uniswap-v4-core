use ethers::types::{Address, U256};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt;

/// Currency represents a token that can be used in the protocol
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Currency {
    /// Native token (ETH on Ethereum)
    Native,
    /// ERC20 token
    Erc20(Address),
    /// Protocol operated token
    Pool(U256),
}

// ZERO_ADDRESS is the address used to represent native ETH
pub const ZERO_ADDRESS: Address = Address::zero();

impl Currency {
    /// Creates a new currency from a token ID
    pub fn from_id(id: U256) -> Self {
        // Simple implementation to convert a U256 ID to a Currency
        // In a real implementation, this would decode the ID appropriately
        Self::Pool(id)
    }
    
    /// Creates a new currency from an address
    pub fn from_address(address: Address) -> Self {
        Self::Erc20(address)
    }
    
    /// Checks if this is the native currency
    pub fn is_native(&self) -> bool {
        matches!(self, Self::Native)
    }
    
    /// Checks if this is an ERC20 token
    pub fn is_erc20(&self) -> bool {
        matches!(self, Self::Erc20(_))
    }
    
    /// Checks if this is a protocol operated token
    pub fn is_pool(&self) -> bool {
        matches!(self, Self::Pool(_))
    }
    
    /// Gets the address of this currency if it's an ERC20
    pub fn address(&self) -> Option<Address> {
        match self {
            Self::Erc20(address) => Some(*address),
            _ => None,
        }
    }
    
    /// Gets the ID of this currency if it's a pool
    pub fn id(&self) -> Option<U256> {
        match self {
            Self::Pool(id) => Some(*id),
            _ => None,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native => write!(f, "Native"),
            Self::Erc20(address) => write!(f, "ERC20({:?})", address),
            Self::Pool(id) => write!(f, "Pool({})", id),
        }
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