pub mod hook_interface;
pub mod hook_registry;
pub mod examples;

pub use hook_interface::*;
pub use hook_registry::*;
pub use examples::*;

/// Result of a before hook call
#[derive(Debug, Clone)]
pub struct BeforeHookResult {
    /// Delta in token0 balance
    pub amount0: i128,
    /// Delta in token1 balance
    pub amount1: i128,
    /// Optional fee override
    pub fee_override: Option<u32>,
}

impl Default for BeforeHookResult {
    fn default() -> Self {
        Self {
            amount0: 0,
            amount1: 0,
            fee_override: None,
        }
    }
}

/// Result of an after hook call
#[derive(Debug, Clone)]
pub struct AfterHookResult {
    /// Delta in token0 balance
    pub amount0: i128,
    /// Delta in token1 balance
    pub amount1: i128,
}

impl Default for AfterHookResult {
    fn default() -> Self {
        Self {
            amount0: 0,
            amount1: 0,
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
    pub const BEFORE_INITIALIZE: u16 = 0x1;
    pub const AFTER_INITIALIZE: u16 = 0x2;
    pub const BEFORE_ADD_LIQUIDITY: u16 = 0x4;
    pub const AFTER_ADD_LIQUIDITY: u16 = 0x8;
    pub const BEFORE_REMOVE_LIQUIDITY: u16 = 0x10;
    pub const AFTER_REMOVE_LIQUIDITY: u16 = 0x20;
    pub const BEFORE_SWAP: u16 = 0x40;
    pub const AFTER_SWAP: u16 = 0x80;
    pub const BEFORE_DONATE: u16 = 0x100;
    pub const AFTER_DONATE: u16 = 0x200;
    
    // Additional flags for hooks that return delta values
    pub const BEFORE_SWAP_RETURNS_DELTA: u16 = 0x400;
    pub const AFTER_SWAP_RETURNS_DELTA: u16 = 0x800;
    pub const AFTER_ADD_LIQUIDITY_RETURNS_DELTA: u16 = 0x1000;
    pub const AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA: u16 = 0x2000;
    
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
}

/// Result type for hook operations
pub type HookResult<T> = std::result::Result<T, HookError>; 