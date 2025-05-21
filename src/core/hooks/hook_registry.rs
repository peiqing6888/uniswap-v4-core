use std::collections::HashMap;
use crate::core::state::{Result as StateResult, BalanceDelta};

use super::{
    hook_interface::{Hook, HookWithReturns, PoolKey, SwapParams, ModifyLiquidityParams},
    HookFlags, BeforeSwapDelta, HookResult, HookError,
};

/// Registry for hooks
pub struct HookRegistry {
    /// Mapping of hook addresses to hook implementations
    hooks: HashMap<[u8; 20], Box<dyn HookWithReturns>>,
}

impl HookRegistry {
    /// Creates a new hook registry
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    /// Registers a hook with the given address
    pub fn register_hook(&mut self, address: [u8; 20], hook: Box<dyn HookWithReturns>) {
        self.hooks.insert(address, hook);
    }

    /// Gets a hook by address
    pub fn get_hook(&mut self, address: &[u8; 20]) -> Option<&mut Box<dyn HookWithReturns>> {
        self.hooks.get_mut(address)
    }

    /// Checks if a hook is registered
    pub fn has_hook(&self, address: &[u8; 20]) -> bool {
        self.hooks.contains_key(address)
    }

    /// Removes a hook from the registry
    pub fn remove_hook(&mut self, address: &[u8; 20]) -> Option<Box<dyn HookWithReturns>> {
        self.hooks.remove(address)
    }

    /// Checks if a specific hook type is enabled for a pool
    pub fn is_hook_enabled(&self, key: &PoolKey, hook_flag: u16) -> bool {
        let flags = HookFlags::from_address(key.hooks);
        flags.is_enabled(hook_flag)
    }
    
    /// Validates that hook address follows rules
    pub fn validate_hook_address(&self, address: &[u8; 20]) -> HookResult<()> {
        let flags = HookFlags::from_address(*address);
        
        if !flags.validate_hook_address() {
            return Err(HookError::HookAddressNotValid(*address));
        }
        
        Ok(())
    }
    
    /// Call a hook that returns a BeforeSwapDelta
    pub fn call_before_swap_with_delta(
        &mut self,
        key: &PoolKey,
        sender: [u8; 20],
        params: &SwapParams,
        hook_data: &[u8],
    ) -> StateResult<BeforeSwapDelta> {
        let flags = HookFlags::from_address(key.hooks);
        
        // Check if we should call this hook and if it returns a delta
        if flags.is_enabled(HookFlags::BEFORE_SWAP) && flags.is_enabled(HookFlags::BEFORE_SWAP_RETURNS_DELTA) {
            if let Some(hook) = self.get_hook(&key.hooks) {
                return hook.before_swap_with_delta(sender, key, params, hook_data);
            }
        }
        
        // Default is no delta
        Ok(BeforeSwapDelta::default())
    }
    
    /// Call a hook that returns an unspecified currency delta after swap
    pub fn call_after_swap_with_delta(
        &mut self,
        key: &PoolKey,
        sender: [u8; 20],
        params: &SwapParams,
        delta: &BalanceDelta,
        hook_data: &[u8],
    ) -> StateResult<i128> {
        let flags = HookFlags::from_address(key.hooks);
        
        // Check if we should call this hook and if it returns a delta
        if flags.is_enabled(HookFlags::AFTER_SWAP) && flags.is_enabled(HookFlags::AFTER_SWAP_RETURNS_DELTA) {
            if let Some(hook) = self.get_hook(&key.hooks) {
                return hook.after_swap_with_delta(sender, key, params, delta, hook_data);
            }
        }
        
        // Default is no delta
        Ok(0)
    }
    
    /// Call a hook that returns a BalanceDelta after adding liquidity
    pub fn call_after_add_liquidity_with_delta(
        &mut self,
        key: &PoolKey,
        sender: [u8; 20],
        params: &ModifyLiquidityParams,
        delta: &BalanceDelta,
        fees_accrued: &BalanceDelta,
        hook_data: &[u8],
    ) -> StateResult<BalanceDelta> {
        let flags = HookFlags::from_address(key.hooks);
        
        // Check if we should call this hook and if it returns a delta
        if flags.is_enabled(HookFlags::AFTER_ADD_LIQUIDITY) && flags.is_enabled(HookFlags::AFTER_ADD_LIQUIDITY_RETURNS_DELTA) {
            if let Some(hook) = self.get_hook(&key.hooks) {
                return hook.after_add_liquidity_with_delta(sender, key, params, delta, fees_accrued, hook_data);
            }
        }
        
        // Default is no delta
        Ok(BalanceDelta { amount0: 0, amount1: 0 })
    }
    
    /// Call a hook that returns a BalanceDelta after removing liquidity
    pub fn call_after_remove_liquidity_with_delta(
        &mut self,
        key: &PoolKey,
        sender: [u8; 20],
        params: &ModifyLiquidityParams,
        delta: &BalanceDelta,
        fees_accrued: &BalanceDelta,
        hook_data: &[u8],
    ) -> StateResult<BalanceDelta> {
        let flags = HookFlags::from_address(key.hooks);
        
        // Check if we should call this hook and if it returns a delta
        if flags.is_enabled(HookFlags::AFTER_REMOVE_LIQUIDITY) && flags.is_enabled(HookFlags::AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA) {
            if let Some(hook) = self.get_hook(&key.hooks) {
                return hook.after_remove_liquidity_with_delta(sender, key, params, delta, fees_accrued, hook_data);
            }
        }
        
        // Default is no delta
        Ok(BalanceDelta { amount0: 0, amount1: 0 })
    }
}

/// A no-op hook that does nothing
pub struct NoOpHook;

impl Hook for NoOpHook {}

impl HookWithReturns for NoOpHook {} 