# Uniswap V4 Core (Rust Implementation)

## Project Status

* [X] Core mathematical libraries implemented
  * [X] FullMath
  * [X] BitMath
  * [X] SqrtPriceMath
  * [X] TickMath
  * [X] LiquidityMath
  * [X] SwapMath
* [X] State management components implemented
  * [X] Pool state
  * [X] Position management
  * [X] Tick tracking
* [X] Core functions implemented
  * [X] Swap execution
  * [X] Liquidity management
  * [X] Fee handling
  * [X] Donate functionality
* [X] Hook system architecture
  * [X] Hook interfaces
  * [X] Hook registry
  * [X] Flag-based hook activation
* [X] Basic pool management
* [X] Comprehensive test suite
  * [X] 34/34 tests passing with full coverage
* [ ] Advanced features (in progress)
  * [ ] Flash loans
  * [ ] Multi-pool routing
  * [ ] Custom hooks implementations
  * [ ] Advanced gas optimizations

This is a Rust implementation of the Uniswap V4 Core protocol, maintaining full compatibility with the original Solidity implementation while leveraging Rust's safety and performance features.

## Recent Updates

### Bug Fixes & Improvements

* Fixed compilation errors related to primitive type conversions
* Resolved mutability issues in BitMath and other core components
* Corrected module visibility problems in the math library
* Fixed tick_math implementation to ensure accurate price/tick conversions
* Enhanced swap functionality to properly handle price limits and direction
* Improved test coverage - all 34 tests now pass successfully

## Project Structure

```
uniswap-v4-core/
├── contracts/                 # Solidity smart contracts
│   ├── interfaces/           # Core interfaces
│   └── core/                 # Essential Solidity components
├── src/                      # Rust implementation
│   ├── core/                 # Core implementation
│   │   ├── math/            # Mathematical operations
│   │   ├── state/           # State management
│   │   ├── hooks/           # Hook system
│   │   └── pool_manager/    # Pool management
│   ├── fees/                # Fee management
│   └── bindings/            # Solidity-Rust bindings
├── tests/                    # Test suite
└── docs/                     # Documentation
```

## Prerequisites

- Rust (latest stable version)
- Foundry (for Solidity components)
- Node.js and npm (for development tools)

## Setup

1. Install Rust dependencies:

   ```bash
   cargo build
   ```
2. Install Foundry dependencies:

   ```bash
   forge install
   ```
3. Run tests:

   ```bash
   # Run Rust tests
   cargo test

   # Run Solidity tests
   forge test
   ```

## Development

This project uses a hybrid approach:

- 90% Rust implementation for core logic and computations
- 10% Solidity for smart contract interfaces and EVM-specific operations

### Key Features

- Full compatibility with Uniswap V4 Core
- Optimized gas efficiency
- Strong type safety through Rust
- Comprehensive test coverage
- FFI layer for Rust-Solidity interaction

## Developer Guidelines

### Common Issues & Best Practices

1. **Type Conversions**: When working with U256 and other numeric types:
   - Use `as_u128()`, `as_i128()` methods for safe conversions
   - Prefer checked arithmetic operations to avoid overflow/underflow
   - Be cautious when shifting bits in large integers to prevent overflow

2. **Mutability Considerations**:
   - Declare variables as mutable (`let mut x`) when they need to be modified
   - Pay special attention to parameters in math functions that modify their inputs

3. **Module Organization**:
   - Ensure modules are properly exported with `pub` when needed by external code
   - Use proper import paths (e.g., `crate::core::math::types`) consistently

4. **Test Strategies**:
   - Test both edge cases and typical scenarios
   - For price calculations, use realistic price values to avoid precision issues
   - When testing swaps, use appropriate price limits based on swap direction

5. **Debugging Tips**:
   - Use `println!()` statements to trace values in tests
   - Compare values against the Solidity implementation for verification
   - Check for off-by-one errors in tick calculations

## Architecture Details

### Math Libraries

Our math libraries provide precise calculations for the core functionality:

- **FullMath**: Handles overflow-safe arithmetic operations
- **BitMath**: Handles bit manipulation and finding positions of bits
- **SqrtPriceMath**: Calculates sqrt price changes and token amounts
- **TickMath**: Converts between sqrt price and tick indices
- **LiquidityMath**: Computes liquidity changes
- **SwapMath**: Calculates swap results and price impact

### State Management

State components track and update pool and user positions:

- **Pool**: Manages pool state, price, liquidity and swap execution
- **Position**: Tracks user positions and accumulated fees
- **Tick**: Handles tick initialization, tracking and crossings

### Hook System

The hook system enables customization of pool behavior:

- **Hook Interface**: Standardized callbacks for pool events
- **Hook Registry**: Central registry for installing and managing hooks
- **Flag-based Activation**: Bitflags determine which hooks are active

## License

GPL-2.0-or-later
