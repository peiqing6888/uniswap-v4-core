pub mod hook_interface;
pub mod hook_registry;
pub mod examples;

use crate::core::state::BalanceDelta;
use ethers::types::Address;

pub use hook_interface::*;
pub use hook_registry::*;
pub use examples::*;

/// Result of a before hook call
#[derive(Debug, Clone)]
pub struct BeforeHookResult {
    /// Optional modified amount
    pub amount: Option<i128>,
    /// Optional balance delta
    pub delta: Option<BalanceDelta>,
    /// Optional fee override
    pub fee_override: Option<u32>,
}

impl Default for BeforeHookResult {
    fn default() -> Self {
        Self {
            amount: None,
            delta: None,
            fee_override: None,
        }
    }
}

/// Result of an after hook call
#[derive(Debug, Clone)]
pub struct AfterHookResult {
    /// Optional balance delta
    pub delta: Option<BalanceDelta>,
}

impl Default for AfterHookResult {
    fn default() -> Self {
        Self {
            delta: None,
        }
    }
}

/// BeforeSwapDelta represents the hook's delta in specified and unspecified currencies
#[derive(Debug, Clone, Default)]
pub struct BeforeSwapDelta {
    /// Delta in specified currency (positive means hook is owed, negative means hook owes)
    pub delta_specified: i128,
    /// Delta in unspecified currency (positive means hook is owed, negative means hook owes)
    pub delta_unspecified: i128,
}

/// Flags for determining which hooks are enabled
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HookFlags(u16);

impl HookFlags {
    pub const BEFORE_INITIALIZE: u16 = 0x1 << 13;
    pub const AFTER_INITIALIZE: u16 = 0x1 << 12;
    pub const BEFORE_ADD_LIQUIDITY: u16 = 0x1 << 11;
    pub const AFTER_ADD_LIQUIDITY: u16 = 0x1 << 10;
    pub const BEFORE_REMOVE_LIQUIDITY: u16 = 0x1 << 9;
    pub const AFTER_REMOVE_LIQUIDITY: u16 = 0x1 << 8;
    pub const BEFORE_SWAP: u16 = 0x1 << 7;
    pub const AFTER_SWAP: u16 = 0x1 << 6;
    pub const BEFORE_DONATE: u16 = 0x1 << 5;
    pub const AFTER_DONATE: u16 = 0x1 << 4;
    
    // Additional flags for hooks that return delta values
    pub const BEFORE_SWAP_RETURNS_DELTA: u16 = 0x1 << 3;
    pub const AFTER_SWAP_RETURNS_DELTA: u16 = 0x1 << 2;
    pub const AFTER_ADD_LIQUIDITY_RETURNS_DELTA: u16 = 0x1 << 1;
    pub const AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA: u16 = 0x1 << 0;
    
    // Mask for all hooks
    pub const ALL_HOOK_MASK: u16 = 0x3FFF; // Covers all 14 hooks

    /// Creates a new set of hook flags from a raw value
    pub fn new(flags: u16) -> Self {
        Self(flags)
    }

    /// Creates a set of hook flags from an address
    pub fn from_address(address: [u8; 20]) -> Self {
        let flags = u16::from_le_bytes([address[0], address[1]]);
        Self(flags)
    }

    /// Checks if a specific hook is enabled
    pub fn is_enabled(&self, flag: u16) -> bool {
        (self.0 & flag) != 0
    }
    
    /// Validates that hook addresses follow the correct pattern
    /// For example, if a hook has BEFORE_SWAP_RETURNS_DELTA, it must also have BEFORE_SWAP
    pub fn validate_hook_address(&self) -> bool {
        // Check if hook has the return delta flag, it must also have the corresponding action flag
        if self.is_enabled(Self::BEFORE_SWAP_RETURNS_DELTA) && !self.is_enabled(Self::BEFORE_SWAP) {
            return false;
        }
        
        if self.is_enabled(Self::AFTER_SWAP_RETURNS_DELTA) && !self.is_enabled(Self::AFTER_SWAP) {
            return false;
        }
        
        if self.is_enabled(Self::AFTER_ADD_LIQUIDITY_RETURNS_DELTA) && !self.is_enabled(Self::AFTER_ADD_LIQUIDITY) {
            return false;
        }
        
        if self.is_enabled(Self::AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA) && !self.is_enabled(Self::AFTER_REMOVE_LIQUIDITY) {
            return false;
        }
        
        true
    }
    
    /// Checks if any hook flag is enabled
    pub fn has_any_hook(&self) -> bool {
        (self.0 & Self::ALL_HOOK_MASK) > 0
    }
    
    /// Validates hook permissions against expected permissions
    pub fn validate_hook_permissions(&self, expected: HookPermissions) -> HookResult<()> {
        if expected.before_initialize != self.is_enabled(Self::BEFORE_INITIALIZE)
            || expected.after_initialize != self.is_enabled(Self::AFTER_INITIALIZE)
            || expected.before_add_liquidity != self.is_enabled(Self::BEFORE_ADD_LIQUIDITY)
            || expected.after_add_liquidity != self.is_enabled(Self::AFTER_ADD_LIQUIDITY)
            || expected.before_remove_liquidity != self.is_enabled(Self::BEFORE_REMOVE_LIQUIDITY)
            || expected.after_remove_liquidity != self.is_enabled(Self::AFTER_REMOVE_LIQUIDITY)
            || expected.before_swap != self.is_enabled(Self::BEFORE_SWAP)
            || expected.after_swap != self.is_enabled(Self::AFTER_SWAP)
            || expected.before_donate != self.is_enabled(Self::BEFORE_DONATE)
            || expected.after_donate != self.is_enabled(Self::AFTER_DONATE)
            || expected.before_swap_returns_delta != self.is_enabled(Self::BEFORE_SWAP_RETURNS_DELTA)
            || expected.after_swap_returns_delta != self.is_enabled(Self::AFTER_SWAP_RETURNS_DELTA)
            || expected.after_add_liquidity_returns_delta != self.is_enabled(Self::AFTER_ADD_LIQUIDITY_RETURNS_DELTA)
            || expected.after_remove_liquidity_returns_delta != self.is_enabled(Self::AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA)
        {
            return Err(HookError::HookAddressNotValid([0u8; 20])); // 实际实现中应该传入真实的地址
        }
        
        Ok(())
    }
    
    /// Checks if the hook address is valid for a given fee
    pub fn is_valid_hook_address(&self, fee: u32, hook_address: Option<Address>) -> bool {
        match hook_address {
            // 如果没有hook合约，则fee不能是动态fee
            None => !is_dynamic_fee(fee),
            // 如果有hook合约，它必须至少有一个flag设置，或者有动态fee
            Some(_) => self.has_any_hook() || is_dynamic_fee(fee)
        }
    }
}

/// Helper function to check if a fee is dynamic
pub fn is_dynamic_fee(fee: u32) -> bool {
    // 在Solidity版本中，动态费用是通过检查fee的最高位来确定的
    (fee & 0x800000) != 0
}

/// Permissions structure for hooks
#[derive(Debug, Clone, Default)]
pub struct HookPermissions {
    pub before_initialize: bool,
    pub after_initialize: bool,
    pub before_add_liquidity: bool,
    pub after_add_liquidity: bool,
    pub before_remove_liquidity: bool,
    pub after_remove_liquidity: bool,
    pub before_swap: bool,
    pub after_swap: bool,
    pub before_donate: bool,
    pub after_donate: bool,
    pub before_swap_returns_delta: bool,
    pub after_swap_returns_delta: bool,
    pub after_add_liquidity_returns_delta: bool,
    pub after_remove_liquidity_returns_delta: bool,
}

/// Error types for hook operations
#[derive(Debug, thiserror::Error)]
pub enum HookError {
    #[error("Hook address not valid: {0:?}")]
    HookAddressNotValid([u8; 20]),
    
    #[error("Invalid hook response")]
    InvalidHookResponse,
    
    #[error("Hook call failed")]
    HookCallFailed,
    
    #[error("Hook delta exceeds swap amount")]
    HookDeltaExceedsSwapAmount,
    
    #[error("Hook call reverted: {0}")]
    HookCallReverted(String),
}

/// Result type for hook operations
pub type HookResult<T> = std::result::Result<T, HookError>; 