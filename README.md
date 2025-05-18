# Uniswap V4 Core (Rust Implementation)


* [X] The core math library is currently implemented
* [ ] State management and pool management components are being written.

This is a Rust implementation of the Uniswap V4 Core protocol, maintaining full compatibility with the original Solidity implementation while leveraging Rust's safety and performance features.

## Project Structure

```
uniswap-v4-core/
├── contracts/                 # Solidity smart contracts
│   ├── interfaces/           # Core interfaces
│   └── core/                 # Essential Solidity components
├── src/                      # Rust implementation
│   ├── core/                 # Core implementation
│   │   ├── pool/            # Pool management
│   │   ├── math/            # Mathematical operations
│   │   └── state/           # State management
│   ├── hooks/               # Hook system
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

## License

GPL-2.0-or-later
