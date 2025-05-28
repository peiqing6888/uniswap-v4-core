pub mod callback;
pub mod implementation;

use crate::core::{
    hooks::{
        Hook, HookWithReturns, BeforeHookResult, AfterHookResult, BeforeSwapDelta,
        PoolKey, SwapParams, ModifyLiquidityParams, HookFlags
    },
    state::{BalanceDelta, Result as StateResult},
};

pub use callback::*;
pub use implementation::*;

/// Trait for hooks that can be registered with the pool manager
pub trait RegisteredHook: Hook + HookWithReturns {
    /// Get the hook flags for this hook
    fn hook_flags(&self) -> HookFlags;
    
    /// Validate that the hook address has the correct flags
    fn validate_hook_address(&self, address: [u8; 20]) -> bool {
        let flags = HookFlags::from_address(address);
        let expected_flags = self.hook_flags();
        
        // Compare the flags
        (expected_flags.0 & HookFlags::ALL_HOOK_MASK) == (flags.0 & HookFlags::ALL_HOOK_MASK)
    }
}

/// Helper struct for implementing hooks
#[derive(Default)]
pub struct HookPermissionsBuilder {
    permissions: crate::core::hooks::HookPermissions,
}

impl HookPermissionsBuilder {
    /// Create a new hook permissions builder
    pub fn new() -> Self {
        Self {
            permissions: Default::default(),
        }
    }

    /// Enable before initialize hook
    pub fn before_initialize(mut self) -> Self {
        self.permissions.before_initialize = true;
        self
    }

    /// Enable after initialize hook
    pub fn after_initialize(mut self) -> Self {
        self.permissions.after_initialize = true;
        self
    }

    /// Enable before add liquidity hook
    pub fn before_add_liquidity(mut self) -> Self {
        self.permissions.before_add_liquidity = true;
        self
    }

    /// Enable after add liquidity hook
    pub fn after_add_liquidity(mut self) -> Self {
        self.permissions.after_add_liquidity = true;
        self
    }

    /// Enable before remove liquidity hook
    pub fn before_remove_liquidity(mut self) -> Self {
        self.permissions.before_remove_liquidity = true;
        self
    }

    /// Enable after remove liquidity hook
    pub fn after_remove_liquidity(mut self) -> Self {
        self.permissions.after_remove_liquidity = true;
        self
    }

    /// Enable before swap hook
    pub fn before_swap(mut self) -> Self {
        self.permissions.before_swap = true;
        self
    }

    /// Enable after swap hook
    pub fn after_swap(mut self) -> Self {
        self.permissions.after_swap = true;
        self
    }

    /// Enable before donate hook
    pub fn before_donate(mut self) -> Self {
        self.permissions.before_donate = true;
        self
    }

    /// Enable after donate hook
    pub fn after_donate(mut self) -> Self {
        self.permissions.after_donate = true;
        self
    }

    /// Enable before swap returns delta hook
    pub fn before_swap_returns_delta(mut self) -> Self {
        self.permissions.before_swap_returns_delta = true;
        self
    }

    /// Enable after swap returns delta hook
    pub fn after_swap_returns_delta(mut self) -> Self {
        self.permissions.after_swap_returns_delta = true;
        self
    }

    /// Enable after add liquidity returns delta hook
    pub fn after_add_liquidity_returns_delta(mut self) -> Self {
        self.permissions.after_add_liquidity_returns_delta = true;
        self
    }

    /// Enable after remove liquidity returns delta hook
    pub fn after_remove_liquidity_returns_delta(mut self) -> Self {
        self.permissions.after_remove_liquidity_returns_delta = true;
        self
    }

    /// Build the hook permissions
    pub fn build(self) -> crate::core::hooks::HookPermissions {
        self.permissions
    }
    
    /// Convert to HookFlags
    pub fn to_hook_flags(&self) -> HookFlags {
        let mut flags = 0u16;
        
        if self.permissions.before_initialize {
            flags |= HookFlags::BEFORE_INITIALIZE;
        }
        if self.permissions.after_initialize {
            flags |= HookFlags::AFTER_INITIALIZE;
        }
        if self.permissions.before_add_liquidity {
            flags |= HookFlags::BEFORE_ADD_LIQUIDITY;
        }
        if self.permissions.after_add_liquidity {
            flags |= HookFlags::AFTER_ADD_LIQUIDITY;
        }
        if self.permissions.before_remove_liquidity {
            flags |= HookFlags::BEFORE_REMOVE_LIQUIDITY;
        }
        if self.permissions.after_remove_liquidity {
            flags |= HookFlags::AFTER_REMOVE_LIQUIDITY;
        }
        if self.permissions.before_swap {
            flags |= HookFlags::BEFORE_SWAP;
        }
        if self.permissions.after_swap {
            flags |= HookFlags::AFTER_SWAP;
        }
        if self.permissions.before_donate {
            flags |= HookFlags::BEFORE_DONATE;
        }
        if self.permissions.after_donate {
            flags |= HookFlags::AFTER_DONATE;
        }
        if self.permissions.before_swap_returns_delta {
            flags |= HookFlags::BEFORE_SWAP_RETURNS_DELTA;
        }
        if self.permissions.after_swap_returns_delta {
            flags |= HookFlags::AFTER_SWAP_RETURNS_DELTA;
        }
        if self.permissions.after_add_liquidity_returns_delta {
            flags |= HookFlags::AFTER_ADD_LIQUIDITY_RETURNS_DELTA;
        }
        if self.permissions.after_remove_liquidity_returns_delta {
            flags |= HookFlags::AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA;
        }
        
        HookFlags::new(flags)
    }
}
