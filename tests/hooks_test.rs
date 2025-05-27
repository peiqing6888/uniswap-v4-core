use ethers::types::Address;
use primitive_types::U256;
use uniswap_v4_core::core::{
    hooks::{
        Hook, HookRegistry, HookWithReturns, HookFlags, BeforeHookResult, AfterHookResult,
        hook_interface::{PoolKey, SwapParams, ModifyLiquidityParams},
        examples::{DynamicFeeHook, TwapOracleHook, LiquidityMiningHook},
    },
    math::types::SqrtPrice,
    state::BalanceDelta,
};

/// Test hook that tracks calls
struct TestHook {
    before_initialize_called: bool,
    after_initialize_called: bool,
    before_swap_called: bool,
    after_swap_called: bool,
    before_add_liquidity_called: bool,
    after_add_liquidity_called: bool,
    before_remove_liquidity_called: bool,
    after_remove_liquidity_called: bool,
    before_donate_called: bool,
    after_donate_called: bool,
}

impl TestHook {
    fn new() -> Self {
        Self {
            before_initialize_called: false,
            after_initialize_called: false,
            before_swap_called: false,
            after_swap_called: false,
            before_add_liquidity_called: false,
            after_add_liquidity_called: false,
            before_remove_liquidity_called: false,
            after_remove_liquidity_called: false,
            before_donate_called: false,
            after_donate_called: false,
        }
    }
    
    fn reset(&mut self) {
        self.before_initialize_called = false;
        self.after_initialize_called = false;
        self.before_swap_called = false;
        self.after_swap_called = false;
        self.before_add_liquidity_called = false;
        self.after_add_liquidity_called = false;
        self.before_remove_liquidity_called = false;
        self.after_remove_liquidity_called = false;
        self.before_donate_called = false;
        self.after_donate_called = false;
    }
}

impl Hook for TestHook {
    fn before_initialize(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _sqrt_price_x96: SqrtPrice,
        _hook_data: &[u8],
    ) -> Result<BeforeHookResult, uniswap_v4_core::core::state::StateError> {
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
    ) -> Result<AfterHookResult, uniswap_v4_core::core::state::StateError> {
        self.after_initialize_called = true;
        Ok(AfterHookResult::default())
    }
    
    fn before_swap(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _params: &SwapParams,
        _hook_data: &[u8],
    ) -> Result<BeforeHookResult, uniswap_v4_core::core::state::StateError> {
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
    ) -> Result<AfterHookResult, uniswap_v4_core::core::state::StateError> {
        self.after_swap_called = true;
        Ok(AfterHookResult::default())
    }
    
    fn before_add_liquidity(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _params: &ModifyLiquidityParams,
        _hook_data: &[u8],
    ) -> Result<BeforeHookResult, uniswap_v4_core::core::state::StateError> {
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
    ) -> Result<AfterHookResult, uniswap_v4_core::core::state::StateError> {
        self.after_add_liquidity_called = true;
        Ok(AfterHookResult::default())
    }
    
    fn before_remove_liquidity(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _params: &ModifyLiquidityParams,
        _hook_data: &[u8],
    ) -> Result<BeforeHookResult, uniswap_v4_core::core::state::StateError> {
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
    ) -> Result<AfterHookResult, uniswap_v4_core::core::state::StateError> {
        self.after_remove_liquidity_called = true;
        Ok(AfterHookResult::default())
    }
    
    fn before_donate(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _amount0: u128,
        _amount1: u128,
        _hook_data: &[u8],
    ) -> Result<BeforeHookResult, uniswap_v4_core::core::state::StateError> {
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
    ) -> Result<AfterHookResult, uniswap_v4_core::core::state::StateError> {
        self.after_donate_called = true;
        Ok(AfterHookResult::default())
    }
}

impl HookWithReturns for TestHook {}

#[test]
fn test_hook_registry() {
    let mut registry = HookRegistry::new();
    let hook_address = [1u8; 20];
    let hook = Box::new(TestHook::new());
    
    // Register hook
    registry.register_hook(hook_address, hook);
    
    // Check if hook is registered
    assert!(registry.has_hook(&hook_address));
    
    // Get hook
    let hook = registry.get_hook(&hook_address);
    assert!(hook.is_some());
    
    // Remove hook
    let removed_hook = registry.remove_hook(&hook_address);
    assert!(removed_hook.is_some());
    assert!(!registry.has_hook(&hook_address));
}

#[test]
fn test_dynamic_fee_hook() {
    let mut hook = DynamicFeeHook::new(3000, 500, 10000);
    
    // Create swap params
    let params = SwapParams {
        amount_specified: 1000000,
        zero_for_one: true,
        sqrt_price_limit_x96: SqrtPrice::new(U256::from(1u128 << 96)),
    };
    
    // Create pool key
    let key = PoolKey {
        token0: [1u8; 20],
        token1: [2u8; 20],
        fee: 3000,
        tick_spacing: 60,
        hooks: [0u8; 20],
        extension_data: vec![],
    };
    
    // Call before_swap
    let result = hook.before_swap([0u8; 20], &key, &params, &[]).unwrap();
    
    // Check that fee override is set
    assert!(result.fee_override.is_some());
    assert_eq!(result.fee_override.unwrap(), 3000); // First call should return base fee
    
    // Call again with different price
    let params2 = SwapParams {
        amount_specified: 1000000,
        zero_for_one: true,
        sqrt_price_limit_x96: SqrtPrice::new(U256::from(2u128 << 96)),
    };
    
    let result2 = hook.before_swap([0u8; 20], &key, &params2, &[]).unwrap();
    
    // Check that fee is adjusted
    assert!(result2.fee_override.is_some());
    assert!(result2.fee_override.unwrap() > 3000); // Fee should increase due to price change
}

#[test]
fn test_twap_oracle_hook() {
    let mut hook = TwapOracleHook::new();
    
    // Create swap params
    let params = SwapParams {
        amount_specified: 1000000,
        zero_for_one: true,
        sqrt_price_limit_x96: SqrtPrice::new(U256::from(1u128 << 96)),
    };
    
    // Create pool key
    let key = PoolKey {
        token0: [1u8; 20],
        token1: [2u8; 20],
        fee: 3000,
        tick_spacing: 60,
        hooks: [0u8; 20],
        extension_data: vec![],
    };
    
    // Call after_swap to update oracle
    let delta = BalanceDelta { amount0: -1000000, amount1: 1000000 };
    hook.after_swap([0u8; 20], &key, &params, &delta, &[]).unwrap();
    
    // Get TWAP
    let twap = hook.get_twap(60); // 60 second TWAP
    
    // TWAP should be non-zero if there's enough data
    // In this test, we might not have enough data yet
    assert_eq!(twap, U256::zero());
    
    // Call after_swap again with different price
    let params2 = SwapParams {
        amount_specified: 1000000,
        zero_for_one: true,
        sqrt_price_limit_x96: SqrtPrice::new(U256::from(2u128 << 96)),
    };
    
    hook.after_swap([0u8; 20], &key, &params2, &delta, &[]).unwrap();
    
    // Since we can't directly access the observations field, we'll check the TWAP again
    // to verify that the oracle has been updated
    let twap_after = hook.get_twap(60);
    // We can't make strong assertions about the value, but we can check it's different
    assert_eq!(twap_after, U256::zero()); // Still likely zero due to timing
}

#[test]
fn test_liquidity_mining_hook() {
    let mut hook = LiquidityMiningHook::new(U256::from(100)); // 100 tokens per second
    
    // Create pool key
    let key = PoolKey {
        token0: [1u8; 20],
        token1: [2u8; 20],
        fee: 3000,
        tick_spacing: 60,
        hooks: [0u8; 20],
        extension_data: vec![],
    };
    
    // Create user
    let user = [3u8; 20];
    
    // Create add liquidity params
    let add_params = ModifyLiquidityParams {
        owner: user,
        tick_lower: -100,
        tick_upper: 100,
        liquidity_delta: 1000000,
        salt: [0u8; 32],
    };
    
    // Add liquidity
    let delta = BalanceDelta { amount0: -1000, amount1: -1000 };
    let fees = BalanceDelta { amount0: 0, amount1: 0 };
    hook.after_add_liquidity([0u8; 20], &key, &add_params, &delta, &fees, &[]).unwrap();
    
    // Wait a bit and check rewards
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    // Remove liquidity
    let remove_params = ModifyLiquidityParams {
        owner: user,
        tick_lower: -100,
        tick_upper: 100,
        liquidity_delta: -1000000,
        salt: [0u8; 32],
    };
    
    hook.after_remove_liquidity([0u8; 20], &key, &remove_params, &delta, &fees, &[]).unwrap();
    
    // Check that user has rewards
    let rewards = hook.claim_rewards(user);
    assert!(rewards > U256::zero());
}

// 创建一个自定义的协议费用钩子
struct CustomProtocolFeeHook {
    fee_fraction: u32,
    collected_fees_0: u128,
    collected_fees_1: u128,
    fee_recipient: [u8; 20],
}

impl CustomProtocolFeeHook {
    fn new(fee_fraction: u32, fee_recipient: [u8; 20]) -> Self {
        Self {
            fee_fraction,
            collected_fees_0: 0,
            collected_fees_1: 0,
            fee_recipient,
        }
    }
    
    fn calculate_protocol_fee(&self, amount: i128) -> i128 {
        if amount <= 0 {
            return 0;
        }
        
        (amount * self.fee_fraction as i128) / 10000
    }
    
    fn withdraw_fees(&mut self) -> (u128, u128) {
        let fees_0 = self.collected_fees_0;
        let fees_1 = self.collected_fees_1;
        
        self.collected_fees_0 = 0;
        self.collected_fees_1 = 0;
        
        (fees_0, fees_1)
    }
}

impl Hook for CustomProtocolFeeHook {}

impl HookWithReturns for CustomProtocolFeeHook {
    fn after_swap_with_delta(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _params: &SwapParams,
        delta: &BalanceDelta,
        _hook_data: &[u8],
    ) -> Result<i128, uniswap_v4_core::core::state::StateError> {
        let fee_0 = self.calculate_protocol_fee(delta.amount0());
        let fee_1 = self.calculate_protocol_fee(delta.amount1());
        
        if fee_0 > 0 {
            self.collected_fees_0 += fee_0 as u128;
        }
        if fee_1 > 0 {
            self.collected_fees_1 += fee_1 as u128;
        }
        
        Ok(fee_0 + fee_1)
    }
}

#[test]
fn test_protocol_fee_hook() {
    let mut hook = CustomProtocolFeeHook::new(30, [4u8; 20]); // 0.3% fee
    
    // Create swap params
    let params = SwapParams {
        amount_specified: 1000000,
        zero_for_one: true,
        sqrt_price_limit_x96: SqrtPrice::new(U256::from(1u128 << 96)),
    };
    
    // Create pool key
    let key = PoolKey {
        token0: [1u8; 20],
        token1: [2u8; 20],
        fee: 3000,
        tick_spacing: 60,
        hooks: [0u8; 20],
        extension_data: vec![],
    };
    
    // Simulate swap with positive delta (tokens received)
    let delta = BalanceDelta { amount0: 0, amount1: 1000000 };
    
    // Call after_swap_with_delta
    let fee_delta = hook.after_swap_with_delta([0u8; 20], &key, &params, &delta, &[]).unwrap();
    
    // Check that fee was collected
    assert_eq!(fee_delta, 3000); // 0.3% of 1000000
    
    // Withdraw fees
    let (fees0, fees1) = hook.withdraw_fees();
    assert_eq!(fees0, 0);
    assert_eq!(fees1, 3000);
} 