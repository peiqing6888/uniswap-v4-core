use primitive_types::U256;
use std::ops::{Add, Sub, Mul, Div};
use num_traits::Zero;

/// U256 扩展特性
pub trait U256Ext {
    /// 将 U256 转换为 i128，如果超出范围则截断
    fn as_i128(&self) -> i128;
}

impl U256Ext for U256 {
    fn as_i128(&self) -> i128 {
        let u128_value = self.as_u128();
        if u128_value > i128::MAX as u128 {
            i128::MAX
        } else {
            u128_value as i128
        }
    }
}

/// Q64.96 fixed-point number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Q64x96(pub U256);

/// Fixed-point scaling factor
pub const Q96: U256 = U256([96, 0, 0, 0]);

/// Represents price as a square root Q64.96
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SqrtPrice(pub U256);

/// Represents liquidity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Liquidity(pub u128);

impl Zero for Q64x96 {
    fn zero() -> Self {
        Self(U256::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl Zero for SqrtPrice {
    fn zero() -> Self {
        Self(U256::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl Zero for Liquidity {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

// Implement Add for Q64x96
impl Add for Q64x96 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

// Implement Sub for Q64x96
impl Sub for Q64x96 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

// Implement Mul for Q64x96
impl Mul for Q64x96 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0 / Q96)
    }
}

// Implement Div for Q64x96
impl Div for Q64x96 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 * Q96 / other.0)
    }
}

// Implement Add for SqrtPrice
impl Add for SqrtPrice {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

// Implement Sub for SqrtPrice
impl Sub for SqrtPrice {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

// Implement Mul for SqrtPrice
impl Mul for SqrtPrice {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0 / Q96)
    }
}

// Implement Div for SqrtPrice
impl Div for SqrtPrice {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 * Q96 / other.0)
    }
}

// Implement Add for Liquidity
impl Add for Liquidity {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

// Implement Sub for Liquidity
impl Sub for Liquidity {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

// Implement Mul for Liquidity
impl Mul for Liquidity {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

// Implement Div for Liquidity
impl Div for Liquidity {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

// Implement From<u128> for Liquidity
impl From<u128> for Liquidity {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl Q64x96 {
    /// Creates a new Q64.96 from a U256
    pub fn new(value: U256) -> Self {
        Self(value)
    }

    /// Converts to U256
    pub fn to_u256(self) -> U256 {
        self.0
    }
}

impl SqrtPrice {
    /// Creates a new SqrtPrice from a U256
    pub fn new(value: U256) -> Self {
        Self(value)
    }

    /// Converts to U256
    pub fn to_u256(self) -> U256 {
        self.0
    }
    
    /// Converts to u128, truncating if necessary
    pub fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }
}

impl Liquidity {
    /// Creates a new Liquidity from a u128
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    /// Converts to u128
    pub fn to_u128(self) -> u128 {
        self.0
    }

    /// Used as alias for to_u128 to maintain API compatibility
    pub fn as_u128(&self) -> u128 {
        self.0
    }
} 