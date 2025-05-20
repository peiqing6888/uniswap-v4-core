use uniswap_v4_core::core::{
    flash_loan::{
        Currency,
        SimpleFlashLoanExample,
        ArbitrageFlashLoanExample,
        MultiTokenFlashLoanExample,
    },
    PoolManager,
};
use ethers::types::{Address, U256};
use std::sync::Arc;

fn main() {
    println!("Uniswap V4 Flash Loan Examples");
    
    // 创建池子管理器
    let mut pool_manager = PoolManager::new();
    
    // 运行简单的 Flash Loan 示例
    run_simple_flash_loan(&mut pool_manager);
    
    // 运行套利 Flash Loan 示例
    run_arbitrage_flash_loan(&mut pool_manager);
    
    // 运行多币种 Flash Loan 示例
    run_multi_token_flash_loan(&mut pool_manager);
}

fn run_simple_flash_loan(pool_manager: &mut PoolManager) {
    println!("\n=== 简单 Flash Loan 示例 ===");
    
    // 创建一个代币地址
    let token_address = Address::from_low_u64_be(1);
    let currency = Currency::from_address(token_address);
    
    // 创建接收地址
    let recipient = Address::from_low_u64_be(2);
    
    // 借用金额
    let amount = 1000;
    
    println!("借用 {} 个代币 {:?} 到地址 {:?}", amount, currency, recipient);
    
    // 创建 Flash Loan 示例
    let flash_loan = SimpleFlashLoanExample::new(
        currency,
        amount,
        recipient,
    );
    
    // 执行 Flash Loan
    match flash_loan.execute(pool_manager) {
        Ok(_) => println!("Flash loan 成功执行!"),
        Err(e) => println!("Flash loan 失败: {:?}", e),
    }
}

fn run_arbitrage_flash_loan(pool_manager: &mut PoolManager) {
    println!("\n=== 套利 Flash Loan 示例 ===");
    
    // 创建代币地址
    let borrow_token = Address::from_low_u64_be(1);
    let target_token = Address::from_low_u64_be(2);
    
    let borrow_currency = Currency::from_address(borrow_token);
    let target_currency = Currency::from_address(target_token);
    
    // 创建接收地址
    let recipient = Address::from_low_u64_be(3);
    
    // 借用金额
    let amount = 1000;
    
    println!("使用代币 {:?} 进行套利，目标代币 {:?}", borrow_currency, target_currency);
    
    // 创建套利 Flash Loan 示例
    let flash_loan = ArbitrageFlashLoanExample::new(
        borrow_currency,
        amount,
        target_currency,
        recipient,
    );
    
    // 执行套利 Flash Loan
    match flash_loan.execute(pool_manager) {
        Ok(_) => println!("套利 Flash loan 成功执行!"),
        Err(e) => println!("套利 Flash loan 失败: {:?}", e),
    }
}

fn run_multi_token_flash_loan(pool_manager: &mut PoolManager) {
    println!("\n=== 多币种 Flash Loan 示例 ===");
    
    // 创建代币地址
    let token1 = Address::from_low_u64_be(1);
    let token2 = Address::from_low_u64_be(2);
    let token3 = Address::from_low_u64_be(3);
    
    let currency1 = Currency::from_address(token1);
    let currency2 = Currency::from_address(token2);
    let currency3 = Currency::from_address(token3);
    
    // 创建接收地址
    let recipient = Address::from_low_u64_be(4);
    
    println!("借用多种代币进行操作");
    
    // 创建多币种 Flash Loan 示例
    let flash_loan = MultiTokenFlashLoanExample::new(
        recipient,
    )
    .add_loan(currency1, 1000)
    .add_loan(currency2, 2000)
    .add_loan(currency3, 3000);
    
    // 执行多币种 Flash Loan
    match flash_loan.execute(pool_manager) {
        Ok(_) => println!("多币种 Flash loan 成功执行!"),
        Err(e) => println!("多币种 Flash loan 失败: {:?}", e),
    }
} 