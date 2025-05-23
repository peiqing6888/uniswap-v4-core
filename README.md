# Uniswap V4 Core (Rust Implementation)

This is a Rust implementation of the Uniswap V4 Core protocol. It aims to provide an alternative to the original [Solidity implementation of Uniswap V4](https://github.com/Uniswap/v4-core), maintaining full API compatibility while leveraging Rust's renowned safety and performance features. This project allows developers to interact with and build upon the Uniswap V4 ecosystem using Rust.

## Project Goals

The primary goals of this Rust implementation are:

- **Performance & Safety:** Leverage Rust's capabilities to offer a potentially more performant and safer execution environment for Uniswap V4's core logic.
- **Full Compatibility:** Maintain strict compatibility with the Uniswap V4 protocol specifications as defined by the core Solidity contracts.
- **Ecosystem Expansion:** Provide a robust Rust-based alternative for developers and projects within the Uniswap V4 ecosystem who prefer or require Rust.

## Target Audience

This project is primarily aimed at:

- **Rust Developers:** Building decentralized applications, tooling, or infrastructure on Uniswap V4 using Rust.
- **Protocol Integrators:** Seeking a Rust-based interface to the Uniswap V4 core functionalities.
- **Researchers & Auditors:** Analyzing the Uniswap V4 protocol through a Rust implementation, focusing on aspects like performance, safety, and alternative design choices.
- **Node Operators & Infrastructure Providers:** Who may wish to run or incorporate a Rust version of the Uniswap V4 core logic.

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

## Technology Stack

This project is built primarily with Rust and leverages several key technologies:

- **Rust:** The core logic is implemented in Rust for safety and performance.
- **Key Rust Crates (examples):**
  - `ethers-rs`: For Ethereum blockchain interaction and smart contract bindings.
  - `revm`: An EVM implementation in Rust, used for testing and local execution.
- **Foundry:** Utilized for managing Solidity contract interactions and dependencies (e.g., via `forge install` as mentioned in project setup).

## Architecture Overview

The `uniswap-v4-core` Rust implementation is designed with a modular architecture. The `src/` directory contains the core Rust logic, separated into components for pool management, mathematical operations, state handling, the hook system, and fee management. Solidity-Rust bindings (`src/bindings/`) facilitate interaction and compatibility with the broader Ethereum and Uniswap V4 ecosystem.

The project structure is as follows:

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

Uniswap V4 Core is licensed under the Business Source License 1.1 (`BUSL-1.1`), see [BUSL_LICENSE](https://github.com/Uniswap/v4-core/blob/main/licenses/BUSL_LICENSE), and the MIT License (`MIT`), see [MIT_LICENSE](https://github.com/Uniswap/v4-core/blob/main/licenses/MIT_LICENSE). Each file in Uniswap V4 Core states the applicable license type in the header.
