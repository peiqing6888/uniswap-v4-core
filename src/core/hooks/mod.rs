mod hook_interface;
mod hook_registry;

pub use hook_interface::*;
pub use hook_registry::*;

/// Result of a before hook call
#[derive(Debug, Clone)]
pub struct BeforeHookResult {
    /// Delta in token0 balance
    pub delta0: i128,
    /// Delta in token1 balance
    pub delta1: i128,
    /// Optional fee override
    pub fee_override: Option<u32>,
}

impl Default for BeforeHookResult {
    fn default() -> Self {
        Self {
            delta0: 0,
            delta1: 0,
            fee_override: None,
        }
    }
}

/// Result of an after hook call
#[derive(Debug, Clone)]
pub struct AfterHookResult {
    /// Delta in token0 balance
    pub delta0: i128,
    /// Delta in token1 balance
    pub delta1: i128,
}

impl Default for AfterHookResult {
    fn default() -> Self {
        Self {
            delta0: 0,
            delta1: 0,
        }
    }
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
} 