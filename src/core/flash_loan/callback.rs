use crate::core::flash_loan::error::FlashLoanError;

/// Callback interface for flash loans
/// This trait should be implemented by users of flash loans
pub trait FlashLoanCallback {
    /// Called by the pool manager when a flash loan is executed
    /// 
    /// # Arguments
    /// * `data` - Any data passed to the unlock call
    /// 
    /// # Returns
    /// Any data to be returned from the unlock call, or an error
    fn unlock_callback(&mut self, data: &[u8]) -> Result<Vec<u8>, FlashLoanError>;
}

/// Callback that does nothing, useful for testing
pub struct EmptyFlashLoanCallback;

impl FlashLoanCallback for EmptyFlashLoanCallback {
    fn unlock_callback(&mut self, _data: &[u8]) -> Result<Vec<u8>, FlashLoanError> {
        Ok(Vec::new())
    }
} 