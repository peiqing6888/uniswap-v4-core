use ethers::types::Address;
use primitive_types::U256;
use super::Currency;

/// 闪电贷回调参数
#[derive(Debug)]
pub struct FlashCallbackData {
    /// 借款人地址
    pub borrower: Address,
    /// 借款的货币
    pub currency: Currency,
    /// 借款金额
    pub amount: U256,
    /// 应还款金额（包含费用）
    pub repay_amount: U256,
    /// 用户自定义数据
    pub user_data: Vec<u8>,
}

/// 闪电贷结果
#[derive(Debug)]
pub enum FlashLoanResult {
    /// 闪电贷成功
    Success,
    /// 闪电贷失败，带有错误信息
    Failure(String),
} 