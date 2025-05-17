mod types;
mod full_math;
mod sqrt_price_math;
mod bit_math;
mod tick_math;
mod liquidity_math;
mod swap_math;

pub use types::*;
pub use full_math::*;
pub use sqrt_price_math::*;
pub use bit_math::*;
pub use tick_math::*;
pub use liquidity_math::*;
pub use swap_math::*;

/// Common error types for math operations
#[derive(Debug, thiserror::Error)]
pub enum MathError {
    #[error("Math overflow")]
    Overflow,
    
    #[error("Division by zero")]
    DivisionByZero,
    
    #[error("Invalid price")]
    InvalidPrice,
    
    #[error("Invalid liquidity")]
    InvalidLiquidity,
    
    #[error("Price overflow")]
    PriceOverflow,
    
    #[error("Not enough liquidity")]
    NotEnoughLiquidity,
}

/// Result type for math operations
pub type Result<T> = std::result::Result<T, MathError>;
