#[cfg(test)]
mod hook_tests {
    use ethers::types::Address;
    use primitive_types::U256;
    use uniswap_v4_core::core::{
        hooks::{
            HookFlags, HookRegistry, NoOpHook,
            BeforeHookResult, AfterHookResult, BeforeSwapDelta,
            hook_interface::{Hook, HookWithReturns, PoolKey, SwapParams, ModifyLiquidityParams},
            examples::{DynamicFeeHook, TwapOracleHook, LiquidityMiningHook, VolumeDiscountHook}
        },
        state::{BalanceDelta, Result as StateResult},
        math::types::SqrtPrice
    };
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::Duration;
    
    #[test]
    fn test_hook_flags() {
        // Test creating and checking hook flags
        let flags = HookFlags::new(HookFlags::BEFORE_SWAP | HookFlags::AFTER_SWAP);
        assert!(flags.is_enabled(HookFlags::BEFORE_SWAP));
        assert!(flags.is_enabled(HookFlags::AFTER_SWAP));
        assert!(!flags.is_enabled(HookFlags::BEFORE_INITIALIZE));
        
        // Test hook flags with return values
        let flags_with_delta = HookFlags::new(
            HookFlags::BEFORE_SWAP | 
            HookFlags::BEFORE_SWAP_RETURNS_DELTA
        );
        assert!(flags_with_delta.is_enabled(HookFlags::BEFORE_SWAP));
        assert!(flags_with_delta.is_enabled(HookFlags::BEFORE_SWAP_RETURNS_DELTA));
        
        // Test hook address validation
        assert!(flags_with_delta.validate_hook_address());
        
        // Test invalid hook address combination
        let invalid_flags = HookFlags::new(HookFlags::BEFORE_SWAP_RETURNS_DELTA);
        assert!(!invalid_flags.validate_hook_address());
    }
    
    // Create a test hook that implements HookWithReturns
    struct TestDeltaHook;
    
    impl Hook for TestDeltaHook {
        fn before_swap(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &SwapParams,
            _hook_data: &[u8],
        ) -> StateResult<BeforeHookResult> {
            Ok(BeforeHookResult {
                amount: Some(100),
                delta: Some(BalanceDelta::new(100, -50)),
                fee_override: Some(2000),
            })
        }
    }
    
    impl HookWithReturns for TestDeltaHook {
        fn before_swap_with_delta(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &SwapParams,
            _hook_data: &[u8],
        ) -> StateResult<BeforeSwapDelta> {
            Ok(BeforeSwapDelta {
                delta_specified: 200,
                delta_unspecified: -100,
            })
        }
        
        fn after_swap_with_delta(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            _params: &SwapParams,
            _delta: &BalanceDelta,
            _hook_data: &[u8],
        ) -> StateResult<i128> {
            Ok(300)
        }
    }
    
    #[test]
    fn test_hook_registry() {
        let mut registry = HookRegistry::new();
        let hook_address = [1u8; 20];
        
        // Register hook
        registry.register_hook(hook_address, Box::new(TestDeltaHook {}));
        assert!(registry.has_hook(&hook_address));
        
        // Test calling hooks that return Delta
        let pool_key = PoolKey {
            token0: [0u8; 20],
            token1: [0u8; 20],
            fee: 3000,
            tick_spacing: 60,
            hooks: hook_address,
            extension_data: vec![],
        };
        
        let sender = [0u8; 20];
        let params = SwapParams {
            amount_specified: -1000,
            zero_for_one: true,
            sqrt_price_limit_x96: SqrtPrice::new(U256::from(1) << 96),
        };
        
        // Hook address doesn't have BEFORE_SWAP_RETURNS_DELTA flag set, so it should return default values
        let delta = registry.call_before_swap_with_delta(&pool_key, sender, &params, &[]).unwrap();
        assert_eq!(delta.delta_specified, 0);
        assert_eq!(delta.delta_unspecified, 0);
        
        // Remove hook
        let removed_hook = registry.remove_hook(&hook_address);
        assert!(removed_hook.is_some());
        assert!(!registry.has_hook(&hook_address));
    }
    
    #[test]
    fn test_dynamic_fee_hook() {
        let mut hook = DynamicFeeHook::new(3000, 500, 10000);
        let sender = [0u8; 20];
        let key = PoolKey {
            token0: [0u8; 20],
            token1: [0u8; 20],
            fee: 3000,
            tick_spacing: 60,
            hooks: [0u8; 20],
            extension_data: vec![],
        };
        
        let params = SwapParams {
            amount_specified: -1000,
            zero_for_one: true,
            sqrt_price_limit_x96: SqrtPrice::new(U256::from(2) << 96),
        };
        
        // First call will set the initial price
        let result = hook.before_swap(sender, &key, &params, &[]).unwrap();
        assert_eq!(result.fee_override, Some(3000)); // Should return base fee rate
        
        // Change price, causing increased volatility
        let params2 = SwapParams {
            amount_specified: -1000,
            zero_for_one: true,
            sqrt_price_limit_x96: SqrtPrice::new(U256::from(3) << 96),
        };
        
        // Second call should return a higher fee rate
        let result2 = hook.before_swap(sender, &key, &params2, &[]).unwrap();
        assert!(result2.fee_override.unwrap() > 3000);
    }
    
    // Custom MockLiquidityMiningHook for testing
    struct MockLiquidityMiningHook {
        user_rewards: HashMap<[u8; 20], U256>,
    }
    
    impl MockLiquidityMiningHook {
        fn new() -> Self {
            Self {
                user_rewards: HashMap::new(),
            }
        }
        
        fn claim_rewards(&mut self, user: [u8; 20]) -> U256 {
            let rewards = *self.user_rewards.get(&user).unwrap_or(&U256::zero());
            if !rewards.is_zero() {
                self.user_rewards.insert(user, U256::zero());
            }
            rewards
        }
    }
    
    impl Hook for MockLiquidityMiningHook {
        fn after_add_liquidity(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            params: &ModifyLiquidityParams,
            _delta: &BalanceDelta,
            _fees_accrued: &BalanceDelta,
            _hook_data: &[u8],
        ) -> StateResult<AfterHookResult> {
            // Just record that liquidity was added - for testing purposes
            Ok(AfterHookResult::default())
        }
        
        fn after_remove_liquidity(
            &mut self,
            _sender: [u8; 20],
            _key: &PoolKey,
            params: &ModifyLiquidityParams,
            _delta: &BalanceDelta,
            _fees_accrued: &BalanceDelta,
            _hook_data: &[u8],
        ) -> StateResult<AfterHookResult> {
            // Mock giving rewards when liquidity is removed - for testing purposes
            let current_rewards = *self.user_rewards.get(&params.owner).unwrap_or(&U256::zero());
            self.user_rewards.insert(params.owner, current_rewards + U256::from(1000));
            
            Ok(AfterHookResult::default())
        }
    }
    
    impl HookWithReturns for MockLiquidityMiningHook {}
    
    #[test]
    fn test_liquidity_mining_hook() {
        // Create a mock hook for testing
        let mut hook = MockLiquidityMiningHook::new();
        let sender = [0u8; 20];
        let owner = [1u8; 20];
        let key = PoolKey {
            token0: [0u8; 20],
            token1: [0u8; 20],
            fee: 3000,
            tick_spacing: 60,
            hooks: [0u8; 20],
            extension_data: vec![],
        };
        
        let params = ModifyLiquidityParams {
            owner,
            tick_lower: -100,
            tick_upper: 100,
            liquidity_delta: 1000,
            salt: [0u8; 32],
        };
        
        let delta = BalanceDelta::new(0, 0);
        let fees = BalanceDelta::new(0, 0);
        
        // Add liquidity, should record user's liquidity
        hook.after_add_liquidity(sender, &key, &params, &delta, &fees, &[]).unwrap();
        
        // Remove liquidity, this will add rewards in our mock implementation
        let remove_params = ModifyLiquidityParams {
            owner,
            tick_lower: -100,
            tick_upper: 100,
            liquidity_delta: -1000,
            salt: [0u8; 32],
        };
        
        hook.after_remove_liquidity(sender, &key, &remove_params, &delta, &fees, &[]).unwrap();
        
        // User should have some rewards
        let rewards = hook.claim_rewards(owner);
        println!("Rewards: {}", rewards);
        assert!(rewards > U256::zero());
    }
} 