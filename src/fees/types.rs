use primitive_types::U256;
use ethers::types::Address;

/// Maximum protocol fee is 0.1% (1000 pips)
pub const MAX_PROTOCOL_FEE: u16 = 1000;

/// Fee denominator (1,000,000) for fee calculations - represents 100%
pub const PIPS_DENOMINATOR: u32 = 1_000_000;

/// Fee threshold for zero-for-one direction
pub const FEE_0_THRESHOLD: u32 = 1001;

/// Fee threshold for one-for-zero direction
pub const FEE_1_THRESHOLD: u32 = 1001 << 12;

/// Protocol fee type that contains both zero-for-one and one-for-zero fees
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolFee(pub u32);

impl ProtocolFee {
    /// Create a new protocol fee
    pub fn new(zero_for_one: u16, one_for_zero: u16) -> Self {
        let value = (zero_for_one as u32) | ((one_for_zero as u32) << 12);
        Self(value)
    }

    /// Get the fee for zero-for-one swaps
    pub fn get_zero_for_one_fee(&self) -> u16 {
        (self.0 & 0xfff) as u16
    }

    /// Get the fee for one-for-zero swaps
    pub fn get_one_for_zero_fee(&self) -> u16 {
        ((self.0 >> 12) & 0xfff) as u16
    }

    /// Check if this protocol fee is valid
    pub fn is_valid(&self) -> bool {
        (self.0 & 0xfff) < FEE_0_THRESHOLD && (self.0 & 0xfff000) < FEE_1_THRESHOLD
    }

    /// Calculate the swap fee combining protocol fee and LP fee
    /// The protocol fee is taken from the input amount first and then the LP fee is taken from the remaining
    pub fn calculate_swap_fee(&self, direction: bool, lp_fee: u32) -> u32 {
        let protocol_fee = if direction {
            self.get_zero_for_one_fee() as u32
        } else {
            self.get_one_for_zero_fee() as u32
        };

        // protocolFee + lpFee - (protocolFee * lpFee / 1_000_000)
        let numerator = protocol_fee * lp_fee;
        protocol_fee + lp_fee - (numerator / PIPS_DENOMINATOR)
    }
}

/// Represents accrued protocol fees for different currencies
#[derive(Debug, Default)]
pub struct ProtocolFeesAccrued {
    /// Maps currency addresses to accrued fee amounts
    pub fees: std::collections::HashMap<Address, U256>,
}

impl ProtocolFeesAccrued {
    /// Create a new empty protocol fees tracker
    pub fn new() -> Self {
        Self {
            fees: std::collections::HashMap::new(),
        }
    }

    /// Get fees accrued for a specific currency
    pub fn get_fees(&self, currency: Address) -> U256 {
        *self.fees.get(&currency).unwrap_or(&U256::zero())
    }

    /// Update fees for a currency
    pub fn update_fees(&mut self, currency: Address, amount: U256) {
        let current = self.get_fees(currency);
        self.fees.insert(currency, current + amount);
    }

    /// Collect fees for a currency
    pub fn collect_fees(&mut self, currency: Address, amount: U256) -> U256 {
        let current = self.get_fees(currency);
        let amount_to_collect = if amount.is_zero() {
            current
        } else {
            amount.min(current)
        };
        
        if !amount_to_collect.is_zero() {
            self.fees.insert(currency, current - amount_to_collect);
        }
        
        amount_to_collect
    }
} 