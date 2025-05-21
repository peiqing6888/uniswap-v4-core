# Uniswap V4 Flash Loans - Rust Implementation

This document describes the Rust implementation of the Flash Loan functionality in Uniswap V4 Core.

## Overview

Flash Loan is a collateral-free loan mechanism that allows users to borrow tokens in a single transaction, as long as the tokens are returned by the end of the transaction (possibly with a fee). This mechanism is very useful in decentralized finance (DeFi) for scenarios like arbitrage, liquidation, and debt restructuring.

In Uniswap V4, Flash Loans are implemented through the following mechanism:

1. The user calls the `unlock` function to start a transaction
2. In `unlockCallback`, the user can call the `take` function to borrow tokens
3. The user uses the borrowed tokens to execute various operations
4. The user must call the `settle` function to return the tokens before the end of the transaction
5. If all tokens are returned (i.e., all deltas are zero), the transaction succeeds; otherwise, it reverts

## Core Components

### 1. FlashLoanManager

`FlashLoanManager` is the core component of the Flash Loan functionality, responsible for managing token borrowing and repayment:

```rust
pub struct FlashLoanManager {
    /// Token delta tracker
    currency_delta_tracker: SharedCurrencyDeltaTracker,
    /// Lock state
    pub lock: Lock,
    /// Currency reserves (for settlement)
    currency_reserves: CurrencyReserves,
}
```

Main methods:

- `unlock`: Unlocks the pool manager and executes the callback
- `take`: Borrows tokens from the pool
- `settle`: Returns tokens to the pool
- `sync`: Synchronizes token balances
- `get_delta`: Gets the token delta for an address
- `clear`: Clears positive deltas

### 2. Currency

The `Currency` type represents a token:

```rust
pub struct Currency(pub Address);
```

Main methods:

- `is_native`: Checks if it's the native token (ETH)
- `from_address`: Creates a Currency from an address
- `to_id`: Converts to an ERC6909 token ID
- `from_id`: Creates a Currency from a token ID

### 3. CurrencyDeltaTracker

`CurrencyDeltaTracker` is responsible for tracking token deltas for each address:

```rust
pub struct CurrencyDeltaTracker {
    // Maps (address, currency) to delta
    deltas: HashMap<(Address, Currency), i128>,
    // Count of non-zero deltas
    non_zero_delta_count: usize,
}
```

Main methods:

- `get_delta`: Gets the delta for a specific address and currency
- `apply_delta`: Applies a delta to a specific address and currency
- `non_zero_delta_count`: Gets the count of non-zero deltas
- `clear_deltas_for_address`: Clears all deltas for an address
- `clear_all_deltas`: Clears all deltas

### 4. Lock

`Lock` controls the lock state of the pool manager:

```rust
pub struct Lock {
    state: Arc<RwLock<bool>>,
}
```

Main methods:

- `unlock`: Unlocks
- `lock`: Locks
- `is_unlocked`: Checks if it's unlocked

### 5. FlashLoanCallback

`FlashLoanCallback` is a user-implemented callback interface:

```rust
pub trait FlashLoanCallback {
    fn unlock_callback(&mut self, data: &[u8]) -> Result<Vec<u8>, FlashLoanError>;
}
```

## Usage Examples

### Simple Flash Loan

```rust
// Create a simple Flash Loan example
let mut flash_loan = SimpleFlashLoanExample::new(
    pool_manager.clone(),
    currency,
    amount,
    recipient,
);

// Execute Flash Loan
let result = flash_loan.execute();
```

### Arbitrage Flash Loan

```rust
// Create arbitrage Flash Loan example
let mut flash_loan = ArbitrageFlashLoanExample::new(
    pool_manager.clone(),
    borrow_currency,
    amount,
    target_currency,
    recipient,
);

// Execute arbitrage Flash Loan
let result = flash_loan.execute();
```

### Multi-token Flash Loan

```rust
// Create multi-token Flash Loan example
let mut flash_loan = MultiTokenFlashLoanExample::new(
    pool_manager.clone(),
    recipient,
)
.add_loan(currency1, 1000)
.add_loan(currency2, 2000)
.add_loan(currency3, 3000);

// Execute multi-token Flash Loan
let result = flash_loan.execute();
```

## Implementation Details

### Borrowing Tokens

When a user calls the `take` function to borrow tokens, the following operations occur:

1. Check if the pool manager is unlocked
2. Apply a negative delta to the user's address
3. Transfer tokens to the user's address

```rust
pub fn take(&self, currency: Currency, to: Address, amount: u128) -> Result<(), FlashLoanError> {
    // Check if unlocked
    if !self.lock.is_unlocked() {
        return Err(FlashLoanError::ManagerLocked);
    }
    
    // Apply delta (negative because tokens are being borrowed)
    self.currency_delta_tracker.apply_delta(to, currency, -(amount as i128));
    
    // Note: In a real implementation, you would transfer tokens here
    
    Ok(())
}
```

### Returning Tokens

When a user calls the `settle` function to return tokens, the following operations occur:

1. Check if the pool manager is unlocked
2. Determine the currency to settle
3. Calculate the amount paid
4. Apply a positive delta to the user's address

```rust
pub fn settle(&self, recipient: Address, value: U256) -> Result<U256, FlashLoanError> {
    // Check if unlocked
    if !self.lock.is_unlocked() {
        return Err(FlashLoanError::ManagerLocked);
    }
    
    // Determine the currency
    let currency = match self.currency_reserves.get_synced_currency() {
        Some(curr) => curr,
        None => Currency::from_address(ZERO_ADDRESS), // Default to native currency
    };
    
    // Calculate the amount paid
    let paid: U256 = if currency.is_native() {
        // For native currency, value is the amount paid
        value
    } else {
        // For ERC20, calculate from reserves
        // ...
    };
    
    // Apply delta (positive because tokens are being returned)
    self.currency_delta_tracker.apply_delta(
        recipient, 
        currency, 
        paid.low_u128() as i128
    );
    
    Ok(paid)
}
```

### Transaction Completion Check

At the end of the transaction, the system checks if all deltas are zero:

```rust
// Check if all currencies are settled
if self.currency_delta_tracker.non_zero_delta_count() != 0 {
    return Err(FlashLoanError::CurrencyNotSettled);
}
```

If there are any non-zero deltas, the transaction will revert.

## Security Considerations

1. **Reentrancy Attack Prevention**: Using the locking mechanism to prevent reentrancy attacks
2. **Delta Tracking**: Precisely tracking token deltas for each address, ensuring all borrowed tokens are returned
3. **Transaction Atomicity**: The entire Flash Loan process is completed in an atomic transaction, ensuring either complete success or complete failure

## Differences from Solidity Implementation

1. **Memory Management**: The Rust implementation uses Rust's ownership system for memory management, rather than Solidity's garbage collection
2. **Concurrency Control**: The Rust implementation uses `Arc` and `RwLock` for concurrency control
3. **Error Handling**: The Rust implementation uses the `Result` type for error handling, rather than Solidity's revert
4. **Type Safety**: Rust provides stronger type safety guarantees

## Conclusion

This document describes the Rust implementation of the Uniswap V4 Flash Loan functionality. With this implementation, users can utilize Uniswap V4's Flash Loan functionality in a Rust environment, enabling various DeFi application scenarios. 