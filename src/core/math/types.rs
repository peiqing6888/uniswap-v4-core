use primitive_types::{U256, U512};
use std::ops::{Add, Sub, Mul, Div};

/// Q64.96 fixed-point number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Q64x96(pub U256);

/// Fixed-point scaling factor
pub const Q96: U256 = U256([96, 0, 0, 0]);

/// Represents price as a square root Q64.96
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SqrtPrice(pub U256);

/// Represents liquidity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Liquidity(pub U128);

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
}

impl Liquidity {
    /// Creates a new Liquidity from a U128
    pub fn new(value: U128) -> Self {
        Self(value)
    }

    /// Converts to U128
    pub fn to_u128(self) -> U128 {
        self.0
    }
} 