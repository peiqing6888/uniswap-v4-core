use crate::core::flash_loan::error::FlashLoanError;
use ethers::types::Address;
use primitive_types::U256;
use super::{Currency, types::FlashCallbackData, FlashLoanResult};

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

/// 闪电贷回调接口
pub trait FlashCallback {
    /// 处理闪电贷回调
    fn execute_operation(
        &mut self,
        borrower: Address,
        currency: Currency,
        amount: U256,
        fee: U256,
        user_data: &[u8],
    ) -> FlashLoanResult;
}

/// 闪电贷提供者接口
pub trait FlashLoanProvider {
    /// 发起闪电贷
    fn flash_loan(
        &mut self,
        borrower: Address,
        receiver: &mut dyn FlashCallback,
        currency: Currency,
        amount: U256,
        user_data: Vec<u8>,
    ) -> Result<(), String>;
    
    /// 计算闪电贷费用
    fn calculate_flash_loan_fee(&self, amount: U256) -> U256;
}

/// 基本闪电贷回调实现
pub struct NoOpFlashCallback;

impl FlashCallback for NoOpFlashCallback {
    fn execute_operation(
        &mut self,
        _borrower: Address,
        _currency: Currency,
        _amount: U256,
        _fee: U256,
        _user_data: &[u8],
    ) -> FlashLoanResult {
        FlashLoanResult::Success
    }
} 