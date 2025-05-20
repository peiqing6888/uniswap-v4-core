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
    
    // Create pool manager
    let mut pool_manager = PoolManager::new();
    
    // Run simple Flash Loan example
    run_simple_flash_loan(&mut pool_manager);
    
    // Run arbitrage Flash Loan example
    run_arbitrage_flash_loan(&mut pool_manager);
    
    // Run multi-token Flash Loan example
    run_multi_token_flash_loan(&mut pool_manager);
}

fn run_simple_flash_loan(pool_manager: &mut PoolManager) {
    println!("\n=== Simple Flash Loan Example ===");
    
    // Create a token address
    let token_address = Address::from_low_u64_be(1);
    let currency = Currency::from_address(token_address);
    
    // Create recipient address
    let recipient = Address::from_low_u64_be(2);
    
    // Borrow amount
    let amount = 1000;
    
    println!("Borrowing {} tokens of currency {:?} to address {:?}", amount, currency, recipient);
    
    // Create Flash Loan example
    let flash_loan = SimpleFlashLoanExample::new(
        currency,
        amount,
        recipient,
    );
    
    // Execute Flash Loan
    match flash_loan.execute(pool_manager) {
        Ok(_) => println!("Flash loan executed successfully!"),
        Err(e) => println!("Flash loan failed: {:?}", e),
    }
}

fn run_arbitrage_flash_loan(pool_manager: &mut PoolManager) {
    println!("\n=== Arbitrage Flash Loan Example ===");
    
    // Create token addresses
    let borrow_token = Address::from_low_u64_be(1);
    let target_token = Address::from_low_u64_be(2);
    
    let borrow_currency = Currency::from_address(borrow_token);
    let target_currency = Currency::from_address(target_token);
    
    // Create recipient address
    let recipient = Address::from_low_u64_be(3);
    
    // Borrow amount
    let amount = 1000;
    
    println!("Using currency {:?} for arbitrage with target currency {:?}", borrow_currency, target_currency);
    
    // Create arbitrage Flash Loan example
    let flash_loan = ArbitrageFlashLoanExample::new(
        borrow_currency,
        amount,
        target_currency,
        recipient,
    );
    
    // Execute arbitrage Flash Loan
    match flash_loan.execute(pool_manager) {
        Ok(_) => println!("Arbitrage Flash loan executed successfully!"),
        Err(e) => println!("Arbitrage Flash loan failed: {:?}", e),
    }
}

fn run_multi_token_flash_loan(pool_manager: &mut PoolManager) {
    println!("\n=== Multi-token Flash Loan Example ===");
    
    // Create token addresses
    let token1 = Address::from_low_u64_be(1);
    let token2 = Address::from_low_u64_be(2);
    let token3 = Address::from_low_u64_be(3);
    
    let currency1 = Currency::from_address(token1);
    let currency2 = Currency::from_address(token2);
    let currency3 = Currency::from_address(token3);
    
    // Create recipient address
    let recipient = Address::from_low_u64_be(4);
    
    println!("Borrowing multiple tokens for operations");
    
    // Create multi-token Flash Loan example
    let flash_loan = MultiTokenFlashLoanExample::new(
        recipient,
    )
    .add_loan(currency1, 1000)
    .add_loan(currency2, 2000)
    .add_loan(currency3, 3000);
    
    // Execute multi-token Flash Loan
    match flash_loan.execute(pool_manager) {
        Ok(_) => println!("Multi-token Flash loan executed successfully!"),
        Err(e) => println!("Multi-token Flash loan failed: {:?}", e),
    }
} 