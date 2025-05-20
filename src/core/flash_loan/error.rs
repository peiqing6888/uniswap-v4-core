use ethers::types::Address;
use crate::core::flash_loan::currency::Currency;

/// Errors that can occur during flash loan operations
#[derive(Debug, thiserror::Error)]
pub enum FlashLoanError {
    #[error("The pool manager is already unlocked")]
    AlreadyUnlocked,
    
    #[error("The pool manager is locked")]
    ManagerLocked,
    
    #[error("Currency not settled")]
    CurrencyNotSettled,
    
    #[error("ERC20 transfer failed")]
    ERC20TransferFailed,
    
    #[error("Requested amount exceeds available balance")]
    InsufficientBalance,
    
    #[error("Protocol fee controller only")]
    InvalidCaller,
    
    #[error("Protocol fee too large: {0}")]
    ProtocolFeeTooLarge(u32),
    
    #[error("Protocol fee currency synced")]
    ProtocolFeeCurrencySynced,
    
    #[error("Nonzero native value")]
    NonzeroNativeValue,
    
    #[error("Must clear exact positive delta")]
    MustClearExactPositiveDelta,
    
    #[error("Swap amount cannot be zero")]
    SwapAmountCannotBeZero,
    
    #[error("Unauthorized dynamic LP fee update")]
    UnauthorizedDynamicLPFeeUpdate,
    
    #[error("Invalid callback")]
    InvalidCallback,
    
    #[error("Failed to encode/decode data: {0}")]
    EncodingError(String),
    
    #[error("Token error: {0}")]
    TokenError(String),
    
    #[error("{0}")]
    Other(String),
} 