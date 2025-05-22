use primitive_types::U256;
use num_traits::Zero;
use crate::core::math::types::{SqrtPrice, Liquidity};

/// Slot0 stores the most frequently accessed state of the pool
#[derive(Debug, Clone)]
pub struct Slot0 {
    /// The current price of the pool as a sqrt(token1/token0) Q64.96 value
    pub sqrt_price_x96: SqrtPrice,
    /// The current tick
    pub tick: i32,
    /// The current protocol fee as a percentage in hundredths of a bip (i.e. 1e-6)
    pub protocol_fee: u32,
    /// The current LP fee as a percentage in hundredths of a bip (i.e. 1e-6)
    pub lp_fee: u32,
}

/// Info stored for each initialized individual tick
#[derive(Debug, Default, Clone)]
pub struct TickInfo {
    /// The total position liquidity that references this tick
    pub liquidity_gross: Liquidity,
    /// Amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left)
    pub liquidity_net: i128,
    /// Fee growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    pub fee_growth_outside_0_x128: U256,
    pub fee_growth_outside_1_x128: U256,
}

/// Balance changes for a pool
#[derive(Debug, Default, Clone, Copy)]
pub struct BalanceDelta {
    /// Change in token0 balance
    pub amount0: i128,
    /// Change in token1 balance
    pub amount1: i128,
}

impl BalanceDelta {
    /// Creates a new balance delta
    pub fn new(amount0: i128, amount1: i128) -> Self {
        Self { amount0, amount1 }
    }
    
    /// Gets the amount0 delta
    pub fn amount0(&self) -> i128 {
        self.amount0
    }
    
    /// Gets the amount1 delta
    pub fn amount1(&self) -> i128 {
        self.amount1
    }
    
    /// Checks if the delta is zero for both tokens
    pub fn is_zero(&self) -> bool {
        self.amount0 == 0 && self.amount1 == 0
    }
    
    /// Adds another balance delta to this one
    pub fn add(&self, other: &Self) -> Self {
        Self {
            amount0: self.amount0 + other.amount0,
            amount1: self.amount1 + other.amount1,
        }
    }
}

impl std::ops::Add for BalanceDelta {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            amount0: self.amount0 + other.amount0,
            amount1: self.amount1 + other.amount1,
        }
    }
}

/// Position represents a liquidity position owned by someone in a pool
#[derive(Debug, Default, Clone)]
pub struct Position {
    /// The amount of liquidity owned by this position
    pub liquidity: Liquidity,
    /// The fee growth of token0 inside the tick range as of the last mint/burn/poke
    pub fee_growth_inside_0_last_x128: U256,
    /// The fee growth of token1 inside the tick range as of the last mint/burn/poke
    pub fee_growth_inside_1_last_x128: U256,
    /// The fees owed to the position owner in token0
    pub tokens_owed_0: u128,
    /// The fees owed to the position owner in token1
    pub tokens_owed_1: u128,
} 