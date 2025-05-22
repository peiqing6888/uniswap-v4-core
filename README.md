# Uniswap V4 Core (Rust Implementation)

This is a Rust implementation of the Uniswap V4 Core protocol, maintaining full compatibility with the original Solidity implementation while leveraging Rust's safety and performance features.

## Key Features

- **Enhanced Hook System**: Customizable hooks that can dynamically adjust fees, collect protocol fees, and reward liquidity providers
- **Protocol Fee System**: Flexible fee collection with independent settings for different trading directions
- **ERC6909 Token Standard**: Multi-token standard for efficient management of liquidity positions
- **Flash Loans**: Built-in flash loan functionality for capital-efficient operations
- **Dynamic Fee Adjustment**: Market volatility-based fee adjustment for optimal trading conditions

## Running Examples and Tests

### Running Examples

```bash
# Run the ERC6909 token standard example
cargo run --example erc6909_example

# Run the protocol fee example
cargo run --example protocol_fee_example

# Run the flash loan example
cargo run --example flash_loan_example
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific integration tests
cargo test --test integration::comprehensive_features_test

# Run specific unit tests
cargo test --test unit::dynamic_fee_hook_test

# Run tests with verbose output
cargo test -- --nocapture
```

## Test Categories

1. **Unit Tests**: Test individual components in isolation
   - Dynamic Fee Hook Test: Tests fee adjustment based on market volatility
   - Protocol Fee Test: Tests protocol fee collection mechanism
   - ERC6909 Test: Tests multi-token standard implementation

2. **Integration Tests**: Test multiple components working together
   - Comprehensive Features Test: Tests interaction between hooks, protocol fees, and ERC6909 tokens
   - Flash Loan Test: Tests flash loan functionality

## Setup

1. Install Rust dependencies:
   ```bash
   cargo build
   ```

2. Install Foundry dependencies:
   ```bash
   forge install
   ```

## Project Structure

```
uniswap-v4-core/
├── contracts/                 # Solidity smart contracts
├── src/                      # Rust implementation
│   ├── core/                 # Core implementation
│   │   ├── math/            # Mathematical operations
│   │   ├── state/           # State management
│   │   ├── hooks/           # Hook system
│   │   └── pool_manager/    # Pool management
│   ├── fees/                # Fee management
│   └── bindings/            # Solidity-Rust bindings
├── examples/                 # Example programs
├── tests/                    # Test suite
└── docs/                     # Documentation
```

## Project Status

* [X] Core mathematical libraries implemented
* [X] State management components implemented
* [X] Core functions implemented
* [X] Hook system architecture
* [X] Basic pool management
* [X] Comprehensive test suite
* [X] Flash loans
* [X] Multi-pool routing
* [ ] Advanced gas optimizations

## Feature Details

### Enhanced Hook System

The enhanced Hook system supports:

- Hooks that return Delta values, allowing hooks to directly influence transaction results
- Hook flag validation, ensuring hook addresses comply with standards
- Hook registry management
- Example Hook implementations:
  - Dynamic Fee Hook - Adjusts fees based on market volatility
  - Liquidity Mining Hook - Rewards liquidity providers

### Protocol Fee System

The protocol fee system supports:

- Setting and collecting protocol fees
- Independent fee settings for different trading directions (zero-for-one and one-for-zero)
- Integration with LP fees
- Protocol fee controller and management
- Fee calculation and distribution mechanisms

### ERC6909 Token Standard

The ERC6909 multi-token standard supports:

- Multi-token management
- Liquidity tokens
- ERC6909Claims extension - Supports token claim functionality
- Integration with pool state for managing liquidity tokens

## Developer Guidelines

### Common Issues & Best Practices

1. **Type Conversions**: When working with U256 and other numeric types:
   - Use `as_u128()`, `as_i128()` methods for safe conversions
   - Prefer checked arithmetic operations to avoid overflow/underflow

2. **Mutability Considerations**:
   - Declare variables as mutable (`let mut x`) when they need to be modified
   - Pay special attention to parameters in math functions that modify their inputs

3. **Test Strategies**:
   - Test both edge cases and typical scenarios
   - For price calculations, use realistic price values to avoid precision issues
   - When testing swaps, use appropriate price limits based on swap direction

## License

GPL-2.0-or-later
