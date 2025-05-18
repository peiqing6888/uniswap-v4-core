use std::collections::HashMap;
use crate::core::state::Result as StateResult;

use super::{
    hook_interface::{Hook, PoolKey},
    HookFlags,
};

/// Registry for hooks
pub struct HookRegistry {
    /// Mapping of hook addresses to hook implementations
    hooks: HashMap<[u8; 20], Box<dyn Hook>>,
}

impl HookRegistry {
    /// Creates a new hook registry
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    /// Registers a hook with the given address
    pub fn register_hook(&mut self, address: [u8; 20], hook: Box<dyn Hook>) {
        self.hooks.insert(address, hook);
    }

    /// Gets a hook by address
    pub fn get_hook(&mut self, address: &[u8; 20]) -> Option<&mut Box<dyn Hook>> {
        self.hooks.get_mut(address)
    }

    /// Checks if a hook is registered
    pub fn has_hook(&self, address: &[u8; 20]) -> bool {
        self.hooks.contains_key(address)
    }

    /// Removes a hook from the registry
    pub fn remove_hook(&mut self, address: &[u8; 20]) -> Option<Box<dyn Hook>> {
        self.hooks.remove(address)
    }

    /// Checks if a specific hook type is enabled for a pool
    pub fn is_hook_enabled(&self, key: &PoolKey, hook_flag: u16) -> bool {
        let flags = HookFlags::from_address(key.hooks);
        flags.is_enabled(hook_flag)
    }
}

/// A no-op hook that does nothing
pub struct NoOpHook;

impl Hook for NoOpHook {} 