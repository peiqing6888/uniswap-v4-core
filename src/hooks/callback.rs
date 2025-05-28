use crate::core::{
    hooks::{
        Hook, HookWithReturns, BeforeHookResult, AfterHookResult, BeforeSwapDelta,
        PoolKey, SwapParams, ModifyLiquidityParams, HookFlags, HookResult, HookError
    },
    math::types::SqrtPrice,
    state::{BalanceDelta, Result as StateResult},
};

/// Hook callback manager that handles calling the appropriate hook functions
pub struct HookCallbacks;

impl HookCallbacks {
    /// Calls the before_initialize hook if enabled
    pub fn before_initialize<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        sqrt_price_x96: SqrtPrice,
        hook_data: &[u8],
        hook_address: [u8; 20],
    ) -> StateResult<()> {
        let flags = HookFlags::from_address(hook_address);
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok(());
        }
        
        if flags.is_enabled(HookFlags::BEFORE_INITIALIZE) {
            hook.before_initialize(sender, key, sqrt_price_x96, hook_data)?;
        }
        
        Ok(())
    }
    
    /// Calls the after_initialize hook if enabled
    pub fn after_initialize<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        sqrt_price_x96: SqrtPrice,
        tick: i32,
        hook_data: &[u8],
        hook_address: [u8; 20],
    ) -> StateResult<()> {
        let flags = HookFlags::from_address(hook_address);
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok(());
        }
        
        if flags.is_enabled(HookFlags::AFTER_INITIALIZE) {
            hook.after_initialize(sender, key, sqrt_price_x96, tick, hook_data)?;
        }
        
        Ok(())
    }
    
    /// Calls the before_modify_liquidity hook (either add or remove) if enabled
    pub fn before_modify_liquidity<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        params: &ModifyLiquidityParams,
        hook_data: &[u8],
        hook_address: [u8; 20],
    ) -> StateResult<()> {
        let flags = HookFlags::from_address(hook_address);
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok(());
        }
        
        if params.liquidity_delta > 0 && flags.is_enabled(HookFlags::BEFORE_ADD_LIQUIDITY) {
            hook.before_add_liquidity(sender, key, params, hook_data)?;
        } else if params.liquidity_delta <= 0 && flags.is_enabled(HookFlags::BEFORE_REMOVE_LIQUIDITY) {
            hook.before_remove_liquidity(sender, key, params, hook_data)?;
        }
        
        Ok(())
    }
    
    /// Calls the after_modify_liquidity hook (either add or remove) if enabled
    /// Returns (caller_delta, hook_delta)
    pub fn after_modify_liquidity<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        params: &ModifyLiquidityParams,
        delta: BalanceDelta,
        fees_accrued: BalanceDelta,
        hook_data: &[u8],
        hook_address: [u8; 20],
    ) -> StateResult<(BalanceDelta, BalanceDelta)> {
        let flags = HookFlags::from_address(hook_address);
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok((delta, BalanceDelta { amount0: 0, amount1: 0 }));
        }
        
        let mut caller_delta = delta;
        let mut hook_delta = BalanceDelta { amount0: 0, amount1: 0 };
        
        if params.liquidity_delta > 0 {
            if flags.is_enabled(HookFlags::AFTER_ADD_LIQUIDITY) {
                if flags.is_enabled(HookFlags::AFTER_ADD_LIQUIDITY_RETURNS_DELTA) {
                    hook_delta = hook.after_add_liquidity_with_delta(
                        sender, key, params, &delta, &fees_accrued, hook_data
                    )?;
                    // Adjust caller's delta based on hook's delta
                    caller_delta = BalanceDelta {
                        amount0: caller_delta.amount0 - hook_delta.amount0,
                        amount1: caller_delta.amount1 - hook_delta.amount1,
                    };
                } else {
                    hook.after_add_liquidity(sender, key, params, &delta, &fees_accrued, hook_data)?;
                }
            }
        } else {
            if flags.is_enabled(HookFlags::AFTER_REMOVE_LIQUIDITY) {
                if flags.is_enabled(HookFlags::AFTER_REMOVE_LIQUIDITY_RETURNS_DELTA) {
                    hook_delta = hook.after_remove_liquidity_with_delta(
                        sender, key, params, &delta, &fees_accrued, hook_data
                    )?;
                    // Adjust caller's delta based on hook's delta
                    caller_delta = BalanceDelta {
                        amount0: caller_delta.amount0 - hook_delta.amount0,
                        amount1: caller_delta.amount1 - hook_delta.amount1,
                    };
                } else {
                    hook.after_remove_liquidity(sender, key, params, &delta, &fees_accrued, hook_data)?;
                }
            }
        }
        
        Ok((caller_delta, hook_delta))
    }
    
    /// Calls the before_swap hook if enabled
    /// Returns (amount_to_swap, hook_return, lp_fee_override)
    pub fn before_swap<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        params: &SwapParams,
        hook_data: &[u8],
        hook_address: [u8; 20],
        fee: u32,
    ) -> StateResult<(i128, BeforeSwapDelta, Option<u32>)> {
        let flags = HookFlags::from_address(hook_address);
        
        // Default values
        let mut amount_to_swap = params.amount_specified;
        let mut hook_return = BeforeSwapDelta::default();
        let mut lp_fee_override = None;
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok((amount_to_swap, hook_return, lp_fee_override));
        }
        
        if flags.is_enabled(HookFlags::BEFORE_SWAP) {
            // Call the hook
            let result = hook.before_swap(sender, key, params, hook_data)?;
            
            // Check if the fee is dynamic and the hook wants to override it
            if is_dynamic_fee(fee) && result.fee_override.is_some() {
                lp_fee_override = result.fee_override;
            }
            
            // If the hook returns a delta and is allowed to
            if flags.is_enabled(HookFlags::BEFORE_SWAP_RETURNS_DELTA) && result.delta.is_some() {
                // Convert the optional delta to BeforeSwapDelta
                if let Some(delta) = result.delta {
                    hook_return = BeforeSwapDelta {
                        delta_specified: delta.amount0,
                        delta_unspecified: delta.amount1,
                    };
                    
                    // Update the swap amount according to the hook's return
                    let hook_delta_specified = hook_return.delta_specified;
                    if hook_delta_specified != 0 {
                        let exact_input = amount_to_swap < 0;
                        amount_to_swap += hook_delta_specified;
                        
                        // Check that the swap type doesn't change (exact input/output)
                        if (exact_input && amount_to_swap > 0) || (!exact_input && amount_to_swap < 0) {
                            return Err(HookError::HookDeltaExceedsSwapAmount.into());
                        }
                    }
                }
            }
        }
        
        Ok((amount_to_swap, hook_return, lp_fee_override))
    }
    
    /// Calls the after_swap hook if enabled
    /// Returns (swap_delta, hook_delta)
    pub fn after_swap<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        params: &SwapParams,
        swap_delta: BalanceDelta,
        hook_data: &[u8],
        hook_address: [u8; 20],
        before_swap_hook_return: BeforeSwapDelta,
    ) -> StateResult<(BalanceDelta, BalanceDelta)> {
        let flags = HookFlags::from_address(hook_address);
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok((swap_delta, BalanceDelta { amount0: 0, amount1: 0 }));
        }
        
        let mut hook_delta_specified = before_swap_hook_return.delta_specified;
        let mut hook_delta_unspecified = before_swap_hook_return.delta_unspecified;
        
        if flags.is_enabled(HookFlags::AFTER_SWAP) {
            if flags.is_enabled(HookFlags::AFTER_SWAP_RETURNS_DELTA) {
                let delta = hook.after_swap_with_delta(sender, key, params, &swap_delta, hook_data)?;
                hook_delta_unspecified += delta;
            } else {
                hook.after_swap(sender, key, params, &swap_delta, hook_data)?;
            }
        }
        
        let mut hook_delta = BalanceDelta { amount0: 0, amount1: 0 };
        let mut caller_delta = swap_delta;
        
        if hook_delta_unspecified != 0 || hook_delta_specified != 0 {
            // Determine which token is specified based on swap direction
            if (params.amount_specified < 0) == params.zero_for_one {
                hook_delta = BalanceDelta {
                    amount0: hook_delta_specified,
                    amount1: hook_delta_unspecified,
                };
            } else {
                hook_delta = BalanceDelta {
                    amount0: hook_delta_unspecified,
                    amount1: hook_delta_specified,
                };
            }
            
            // Adjust caller's delta based on hook's delta
            caller_delta = BalanceDelta {
                amount0: caller_delta.amount0 - hook_delta.amount0,
                amount1: caller_delta.amount1 - hook_delta.amount1,
            };
        }
        
        Ok((caller_delta, hook_delta))
    }
    
    /// Calls the before_donate hook if enabled
    pub fn before_donate<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        amount0: u128,
        amount1: u128,
        hook_data: &[u8],
        hook_address: [u8; 20],
    ) -> StateResult<()> {
        let flags = HookFlags::from_address(hook_address);
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok(());
        }
        
        if flags.is_enabled(HookFlags::BEFORE_DONATE) {
            hook.before_donate(sender, key, amount0, amount1, hook_data)?;
        }
        
        Ok(())
    }
    
    /// Calls the after_donate hook if enabled
    pub fn after_donate<H: HookWithReturns>(
        hook: &mut H,
        sender: [u8; 20],
        key: &PoolKey,
        amount0: u128,
        amount1: u128,
        hook_data: &[u8],
        hook_address: [u8; 20],
    ) -> StateResult<()> {
        let flags = HookFlags::from_address(hook_address);
        
        // Skip if sender is the hook itself (prevents recursion)
        if sender == hook_address {
            return Ok(());
        }
        
        if flags.is_enabled(HookFlags::AFTER_DONATE) {
            hook.after_donate(sender, key, amount0, amount1, hook_data)?;
        }
        
        Ok(())
    }
}

/// Helper function to check if a fee is dynamic
pub fn is_dynamic_fee(fee: u32) -> bool {
    (fee & 0x800000) != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::math::types::Liquidity;
    
    struct TestHook {
        pub before_initialize_called: bool,
        pub after_initialize_called: bool,
        pub before_add_liquidity_called: bool,
        pub after_add_liquidity_called: bool,
        pub before_remove_liquidity_called: bool,
        pub after_remove_liquidity_called: bool,
        pub before_swap_called: bool,
        pub after_swap_called: bool,
        pub before_donate_called: bool,
        pub after_donate_called: bool,
    }
    
    impl Default for TestHook {
        fn default() -> Self {
            Self {
                before_initialize_called: false,
                after_initialize_called: false,
                before_add_liquidity_called: false,
                after_add_liquidity_called: false,
                before_remove_liquidity_called: false,
                after_remove_liquidity_called: false,
                before_swap_called: false,
                after_swap_called: false,
                before_donate_called: false,
                after_donate_called: false,
            }
        }
    }
    
    impl Hook for TestHook {
        fn before_initialize(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _sqrt_price_x96: SqrtPrice,
            _hook_data: &[u8],
        ) -> StateResult<BeforeHookResult> {
            self.before_initialize_called = true;
            Ok(BeforeHookResult::default())
        }
        
        fn after_initialize(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _sqrt_price_x96: SqrtPrice,
            _tick: i32,
            _hook_data: &[u8],
        ) -> StateResult<AfterHookResult> {
            self.after_initialize_called = true;
            Ok(AfterHookResult::default())
        }
        
        fn before_add_liquidity(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &ModifyLiquidityParams,
            _hook_data: &[u8],
        ) -> StateResult<BeforeHookResult> {
            self.before_add_liquidity_called = true;
            Ok(BeforeHookResult::default())
        }
        
        fn after_add_liquidity(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &ModifyLiquidityParams,
            _delta: &BalanceDelta,
            _fees_accrued: &BalanceDelta,
            _hook_data: &[u8],
        ) -> StateResult<AfterHookResult> {
            self.after_add_liquidity_called = true;
            Ok(AfterHookResult::default())
        }
        
        fn before_remove_liquidity(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &ModifyLiquidityParams,
            _hook_data: &[u8],
        ) -> StateResult<BeforeHookResult> {
            self.before_remove_liquidity_called = true;
            Ok(BeforeHookResult::default())
        }
        
        fn after_remove_liquidity(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &ModifyLiquidityParams,
            _delta: &BalanceDelta,
            _fees_accrued: &BalanceDelta,
            _hook_data: &[u8],
        ) -> StateResult<AfterHookResult> {
            self.after_remove_liquidity_called = true;
            Ok(AfterHookResult::default())
        }
        
        fn before_swap(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &SwapParams,
            _hook_data: &[u8],
        ) -> StateResult<BeforeHookResult> {
            self.before_swap_called = true;
            Ok(BeforeHookResult::default())
        }
        
        fn after_swap(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &SwapParams,
            _delta: &BalanceDelta,
            _hook_data: &[u8],
        ) -> StateResult<AfterHookResult> {
            self.after_swap_called = true;
            Ok(AfterHookResult::default())
        }
        
        fn before_donate(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _amount0: u128,
            _amount1: u128,
            _hook_data: &[u8],
        ) -> StateResult<BeforeHookResult> {
            self.before_donate_called = true;
            Ok(BeforeHookResult::default())
        }
        
        fn after_donate(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _amount0: u128,
            _amount1: u128,
            _hook_data: &[u8],
        ) -> StateResult<AfterHookResult> {
            self.after_donate_called = true;
            Ok(AfterHookResult::default())
        }
    }
    
    impl HookWithReturns for TestHook {}
    
    #[test]
    fn test_before_initialize() {
        let mut hook = TestHook::default();
        let sender = [1u8; 20];
        let hook_address = [2u8; 20];
        let key = PoolKey {
            token0: [0u8; 20],
            token1: [0u8; 20],
            fee: 0,
            tick_spacing: 1,
            hooks: hook_address,
            extension_data: vec![],
        };
        let sqrt_price = SqrtPrice::new(primitive_types::U256::from(1u64 << 96));
        
        // Test with BEFORE_INITIALIZE flag enabled
        let hook_address = [0u8, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // 0x2000 = BEFORE_INITIALIZE
        HookCallbacks::before_initialize(&mut hook, sender, &key, sqrt_price, &[], hook_address).unwrap();
        assert!(hook.before_initialize_called);
        
        // Test with flag disabled
        let mut hook = TestHook::default();
        let hook_address = [0u8; 20];
        HookCallbacks::before_initialize(&mut hook, sender, &key, sqrt_price, &[], hook_address).unwrap();
        assert!(!hook.before_initialize_called);
        
        // Test with sender == hook_address (should skip)
        let mut hook = TestHook::default();
        let sender = hook_address;
        HookCallbacks::before_initialize(&mut hook, sender, &key, sqrt_price, &[], hook_address).unwrap();
        assert!(!hook.before_initialize_called);
    }
} 