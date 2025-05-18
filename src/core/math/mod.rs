pub mod types;
pub mod sqrt_price_math;
pub mod full_math;
pub mod tick_math;
pub mod liquidity_math;
pub mod swap_math;
pub mod bit_math;

pub use types::*;
pub use sqrt_price_math::*;
pub use full_math::*;
pub use tick_math::*;
pub use liquidity_math::*;
pub use swap_math::*;
pub use bit_math::*;

use std::fmt;

/// Math errors
#[derive(Debug, Clone, Copy)]
pub enum MathError {
    /// Overflow error
    Overflow,
    /// Invalid price error
    InvalidPrice,
    /// Invalid tick error
    InvalidTick,
    /// Division by zero error
    DivisionByZero,
    /// Not enough liquidity error
    NotEnoughLiquidity,
    /// Price overflow error
    PriceOverflow,
    /// Invalid liquidity error
    InvalidLiquidity,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Overflow => write!(f, "Overflow"),
            Self::InvalidPrice => write!(f, "Invalid price"),
            Self::InvalidTick => write!(f, "Invalid tick"),
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::NotEnoughLiquidity => write!(f, "Not enough liquidity"),
            Self::PriceOverflow => write!(f, "Price overflow"),
            Self::InvalidLiquidity => write!(f, "Invalid liquidity"),
        }
    }
}

/// Math result type
pub type Result<T> = std::result::Result<T, MathError>;
