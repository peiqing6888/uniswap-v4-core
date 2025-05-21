//! Uniswap V4 Core implementation in Rust
//! This crate provides a Rust implementation of the Uniswap V4 Core protocol

pub mod core {
    pub mod pool;
    pub mod math;
    pub mod state;
    pub mod flash_loan;
    pub mod pool_manager;
    pub mod hooks;
    
    pub use pool_manager::PoolManager;
    pub use flash_loan::*;
    pub use flash_loan::currency::Currency;
}

pub mod hooks;
pub mod fees;
pub mod bindings;
pub mod tokens;

// Re-export commonly used types
pub use ethers;
pub use core::flash_loan::currency::Currency;

/// Common error types for the crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Math error: {0}")]
    Math(String),
    
    #[error("Pool error: {0}")]
    Pool(String),
    
    #[error("State error: {0}")]
    State(String),
    
    #[error("Hook error: {0}")]
    Hook(String),
    
    #[error("FFI error: {0}")]
    FFI(String),
    
    #[error("Flash loan error: {0}")]
    FlashLoan(String),
}

/// Result type for the crate
pub type Result<T> = std::result::Result<T, Error>;

// Initialize logging
#[cfg(not(test))]
pub fn init_logging() {
    use tracing_subscriber::{fmt, EnvFilter};
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
