use crate::core::{
    hooks::{
        Hook, HookWithReturns, BeforeHookResult, AfterHookResult, BeforeSwapDelta, HookFlags,
        PoolKey, SwapParams, ModifyLiquidityParams,
    },
    math::types::SqrtPrice,
    state::{BalanceDelta, Result as StateResult},
};
use super::RegisteredHook;

/// A sample fee hook that dynamically adjusts fees based on market conditions
pub struct DynamicFeeHook {
    /// Base fee in hundredths of a bip (0.0001%)
    base_fee: u32,
    /// Maximum fee in hundredths of a bip
    max_fee: u32,
    /// Volatility factor - how much to increase fee during high volatility
    volatility_factor: u32,
    /// Last price seen by the hook
    last_price: Option<SqrtPrice>,
}

impl DynamicFeeHook {
    /// Create a new dynamic fee hook
    pub fn new(base_fee: u32, max_fee: u32, volatility_factor: u32) -> Self {
        Self {
            base_fee,
            max_fee,
            volatility_factor,
            last_price: None,
        }
    }
    
    /// Calculate the dynamic fee based on price movement
    fn calculate_fee(&mut self, current_price: SqrtPrice) -> u32 {
        let last_price = match self.last_price {
            Some(price) => price,
            None => {
                self.last_price = Some(current_price);
                return self.base_fee;
            }
        };
        
        // Update last price
        self.last_price = Some(current_price);
        
        // Calculate price change percentage
        let price_a = last_price.to_u256();
        let price_b = current_price.to_u256();
        
        if price_a.is_zero() || price_b.is_zero() {
            return self.base_fee;
        }
        
        // Calculate price change as a percentage
        let (larger, smaller) = if price_a > price_b {
            (price_a, price_b)
        } else {
            (price_b, price_a)
        };
        
        // Calculate price change as a factor (larger / smaller)
        let price_change = match larger.checked_mul(1000u64.into()).and_then(|res| res.checked_div(smaller)) {
            Some(change) => change.as_u32().unwrap_or(1000) - 1000, // Convert to percentage points above 100%
            None => 0,
        };
        
        // Adjust fee based on price change
        let additional_fee = (price_change * self.volatility_factor) / 100;
        let dynamic_fee = self.base_fee + additional_fee;
        
        // Cap at max fee
        std::cmp::min(dynamic_fee, self.max_fee)
    }
}

impl Hook for DynamicFeeHook {
    fn before_swap(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _params: &SwapParams,
        _hook_data: &[u8],
    ) -> StateResult<BeforeHookResult> {
        let fee = self.calculate_fee(_params.sqrt_price_limit_x96);
        
        Ok(BeforeHookResult {
            fee_override: Some(fee),
            ..Default::default()
        })
    }
}

impl HookWithReturns for DynamicFeeHook {}

impl RegisteredHook for DynamicFeeHook {
    fn hook_flags(&self) -> HookFlags {
        // Enable only the before_swap hook
        HookFlags::new(HookFlags::BEFORE_SWAP)
    }
}

/// A liquidity mining hook that rewards users for providing liquidity
pub struct LiquidityMiningHook {
    /// Reward token amount per unit of liquidity
    reward_per_liquidity: u128,
    /// Accumulated rewards by user
    rewards: std::collections::HashMap<[u8; 20], u128>,
}

impl LiquidityMiningHook {
    /// Create a new liquidity mining hook
    pub fn new(reward_per_liquidity: u128) -> Self {
        Self {
            reward_per_liquidity,
            rewards: std::collections::HashMap::new(),
        }
    }
    
    /// Calculate rewards based on liquidity provided
    fn calculate_rewards(&self, liquidity: i128) -> u128 {
        if liquidity <= 0 {
            return 0;
        }
        
        (self.reward_per_liquidity * liquidity as u128) / 1_000_000
    }
    
    /// Get accumulated rewards for a user
    pub fn get_rewards(&self, user: &[u8; 20]) -> u128 {
        *self.rewards.get(user).unwrap_or(&0)
    }
}

impl Hook for LiquidityMiningHook {
    fn after_add_liquidity(
        &mut self,
        sender: [u8; 20],
        _key: &PoolKey,
        params: &ModifyLiquidityParams,
        _delta: &BalanceDelta,
        _fees_accrued: &BalanceDelta,
        _hook_data: &[u8],
    ) -> StateResult<AfterHookResult> {
        // Calculate rewards based on liquidity added
        let rewards = self.calculate_rewards(params.liquidity_delta);
        
        // Add rewards to user's balance
        let user_rewards = self.rewards.entry(sender).or_insert(0);
        *user_rewards += rewards;
        
        Ok(AfterHookResult::default())
    }
}

impl HookWithReturns for LiquidityMiningHook {}

impl RegisteredHook for LiquidityMiningHook {
    fn hook_flags(&self) -> HookFlags {
        // Enable only the after_add_liquidity hook
        HookFlags::new(HookFlags::AFTER_ADD_LIQUIDITY)
    }
}

/// A fee sharing hook that takes a portion of swap fees and distributes them to a beneficiary
pub struct FeeSharingHook {
    /// Beneficiary address
    beneficiary: [u8; 20],
    /// Fee share percentage (0-100)
    fee_share_percent: u8,
    /// Accumulated fees
    accumulated_fees: BalanceDelta,
}

impl FeeSharingHook {
    /// Create a new fee sharing hook
    pub fn new(beneficiary: [u8; 20], fee_share_percent: u8) -> Self {
        let fee_share_percent = std::cmp::min(fee_share_percent, 100);
        
        Self {
            beneficiary,
            fee_share_percent,
            accumulated_fees: BalanceDelta { amount0: 0, amount1: 0 },
        }
    }
    
    /// Get the beneficiary address
    pub fn beneficiary(&self) -> [u8; 20] {
        self.beneficiary
    }
    
    /// Get accumulated fees
    pub fn accumulated_fees(&self) -> &BalanceDelta {
        &self.accumulated_fees
    }
}

impl Hook for FeeSharingHook {}

impl HookWithReturns for FeeSharingHook {
    fn after_swap_with_delta(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        params: &SwapParams,
        delta: &BalanceDelta,
        _hook_data: &[u8],
    ) -> StateResult<i128> {
        // Calculate fee share
        let fee_share_0 = (delta.amount0.abs() as u128 * self.fee_share_percent as u128) / 100;
        let fee_share_1 = (delta.amount1.abs() as u128 * self.fee_share_percent as u128) / 100;
        
        // Update accumulated fees
        self.accumulated_fees.amount0 += fee_share_0 as i128;
        self.accumulated_fees.amount1 += fee_share_1 as i128;
        
        // Return delta for the unspecified token
        if params.zero_for_one {
            Ok(fee_share_1 as i128)
        } else {
            Ok(fee_share_0 as i128)
        }
    }
}

impl RegisteredHook for FeeSharingHook {
    fn hook_flags(&self) -> HookFlags {
        // Enable after_swap and after_swap_returns_delta hooks
        HookFlags::new(HookFlags::AFTER_SWAP | HookFlags::AFTER_SWAP_RETURNS_DELTA)
    }
}

/// A price oracle hook that tracks price movements
pub struct PriceOracleHook {
    /// Historical prices (timestamp -> price)
    prices: std::collections::VecDeque<(u64, SqrtPrice)>,
    /// Maximum number of price points to store
    max_history: usize,
    /// Current timestamp provider
    timestamp_provider: Box<dyn Fn() -> u64>,
}

impl PriceOracleHook {
    /// Create a new price oracle hook
    pub fn new(max_history: usize, timestamp_provider: Box<dyn Fn() -> u64>) -> Self {
        Self {
            prices: std::collections::VecDeque::with_capacity(max_history),
            max_history,
            timestamp_provider,
        }
    }
    
    /// Get the current timestamp
    fn now(&self) -> u64 {
        (self.timestamp_provider)()
    }
    
    /// Get the time-weighted average price over a period
    pub fn get_twap(&self, period_seconds: u64) -> Option<SqrtPrice> {
        if self.prices.is_empty() {
            return None;
        }
        
        let now = self.now();
        let start_time = now.saturating_sub(period_seconds);
        
        // Find prices within the period
        let prices_in_period: Vec<_> = self.prices
            .iter()
            .filter(|(ts, _)| *ts >= start_time)
            .collect();
        
        if prices_in_period.is_empty() {
            return None;
        }
        
        // Calculate time-weighted average
        let mut sum_weighted_price = primitive_types::U256::zero();
        let mut sum_weights = 0u64;
        
        for i in 0..prices_in_period.len() {
            let (ts, price) = prices_in_period[i];
            let next_ts = if i < prices_in_period.len() - 1 {
                prices_in_period[i + 1].0
            } else {
                now
            };
            
            let weight = next_ts - *ts;
            sum_weighted_price += price.to_u256() * weight;
            sum_weights += weight;
        }
        
        if sum_weights == 0 {
            return None;
        }
        
        Some(SqrtPrice::new(sum_weighted_price / sum_weights))
    }
}

impl Hook for PriceOracleHook {
    fn after_swap(
        &mut self,
        _sender: [u8; 20],
        _key: &PoolKey,
        _params: &SwapParams,
        _delta: &BalanceDelta,
        _hook_data: &[u8],
    ) -> StateResult<AfterHookResult> {
        // Record the current price
        let now = self.now();
        let current_price = _params.sqrt_price_limit_x96;
        
        self.prices.push_back((now, current_price));
        
        // Maintain max history size
        while self.prices.len() > self.max_history {
            self.prices.pop_front();
        }
        
        Ok(AfterHookResult::default())
    }
}

impl HookWithReturns for PriceOracleHook {}

impl RegisteredHook for PriceOracleHook {
    fn hook_flags(&self) -> HookFlags {
        // Enable only the after_swap hook
        HookFlags::new(HookFlags::AFTER_SWAP)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U256;
    
    #[test]
    fn test_dynamic_fee_hook() {
        let mut hook = DynamicFeeHook::new(500, 10000, 200);
        
        // Initial fee should be base fee
        let price1 = SqrtPrice::new(U256::from(1_000_000));
        let fee1 = hook.calculate_fee(price1);
        assert_eq!(fee1, 500);
        
        // Price change of 10% should increase fee
        let price2 = SqrtPrice::new(U256::from(1_100_000));
        let fee2 = hook.calculate_fee(price2);
        assert!(fee2 > 500);
        
        // Test hook flags
        let flags = hook.hook_flags();
        assert!(flags.is_enabled(HookFlags::BEFORE_SWAP));
        assert!(!flags.is_enabled(HookFlags::AFTER_SWAP));
    }
    
    #[test]
    fn test_liquidity_mining_hook() {
        let mut hook = LiquidityMiningHook::new(1000);
        let user = [1u8; 20];
        
        // No rewards initially
        assert_eq!(hook.get_rewards(&user), 0);
        
        // Add liquidity and check rewards
        let params = ModifyLiquidityParams {
            owner: user,
            tick_lower: 0,
            tick_upper: 100,
            liquidity_delta: 1_000_000,
            salt: [0u8; 32],
        };
        
        let delta = BalanceDelta { amount0: 100, amount1: 200 };
        let fees = BalanceDelta { amount0: 0, amount1: 0 };
        
        hook.after_add_liquidity(user, &PoolKey::default(), &params, &delta, &fees, &[]).unwrap();
        
        // Should have rewards now
        assert!(hook.get_rewards(&user) > 0);
    }
}

impl Default for PoolKey {
    fn default() -> Self {
        Self {
            token0: [0u8; 20],
            token1: [0u8; 20],
            fee: 0,
            tick_spacing: 1,
            hooks: [0u8; 20],
            extension_data: vec![],
        }
    }
} 