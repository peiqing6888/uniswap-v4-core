mod pool;
mod position;
mod tick;
mod types;

pub use pool::*;
pub use position::*;
pub use tick::*;
pub use types::*;

use thiserror::Error;

/// Common error types for state operations
#[derive(Debug, Error)]
pub enum StateError {
    #[error("Ticks misordered: lower {0}, upper {1}")]
    TicksMisordered(i32, i32),

    #[error("Tick lower out of bounds: {0}")]
    TickLowerOutOfBounds(i32),

    #[error("Tick upper out of bounds: {0}")]
    TickUpperOutOfBounds(i32),

    #[error("Tick liquidity overflow at tick {0}")]
    TickLiquidityOverflow(i32),

    #[error("Pool already initialized")]
    PoolAlreadyInitialized,

    #[error("Pool not initialized")]
    PoolNotInitialized,

    #[error("Price limit already exceeded: current {0}, limit {1}")]
    PriceLimitAlreadyExceeded(u128, u128),

    #[error("Price limit out of bounds: {0}")]
    PriceLimitOutOfBounds(u128),

    #[error("No liquidity to receive fees")]
    NoLiquidityToReceiveFees,

    #[error("Invalid fee for exact out")]
    InvalidFeeForExactOut,

    #[error("Invalid price")]
    InvalidPrice,
}

/// Result type for state operations
pub type Result<T> = std::result::Result<T, StateError>;
