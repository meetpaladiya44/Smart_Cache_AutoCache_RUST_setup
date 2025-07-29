# Stylus Cache SDK

A Rust SDK for Stylus contract caching utilities with AutoCacheOptIn event support.

## Features

- 🚀 **Easy Integration**: Simple functions for contract caching
- 📝 **Event Support**: Built-in `AutoCacheOptIn` event
- 🛠️ **Trait-based**: Optional `Cacheable` trait for standardized implementation
- 📚 **Well Documented**: Comprehensive documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
stylus-cache-sdk = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use stylus_cache_sdk::{is_contract_cacheable, AutoCacheOptIn, emit_cache_opt_in};
use stylus_sdk::prelude::*;
use alloy_primitives::Address;

#[public]
impl MyContract {
    pub fn is_cacheable(&self) -> bool {
        is_contract_cacheable()
    }
    
    pub fn opt_into_cache(&self, contract_addr: Address) {
        emit_cache_opt_in(contract_addr);
    }
}
```

### Using the Cacheable Trait

```rust
use stylus_cache_sdk::Cacheable;
use stylus_sdk::prelude::*;
use alloy_primitives::Address;

#[public]
impl MyContract {
    // Contract implementation
}

impl Cacheable for MyContract {}

// Now you can use:
// contract.is_cacheable() -> bool
// contract.opt_into_cache(address) -> emits AutoCacheOptIn event
```

### Updated Counter Contract Example

```rust
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*};
use stylus_cache_sdk::{is_contract_cacheable, Cacheable};

sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

#[public]
impl Counter {
    pub fn is_cacheable(&self) -> bool {
        is_contract_cacheable()
    }

    pub fn numbers(&self) -> U256 {
        self.number.get()
    }

    // ... rest of your contract methods
}

impl Cacheable for Counter {}
```

## API Reference

### Functions

- `is_contract_cacheable() -> bool`: Returns whether the contract is cacheable
- `emit_cache_opt_in(contract_addr: Address)`: Emits the AutoCacheOptIn event

### Events

- `AutoCacheOptIn(address indexed contract_addr)`: Event for cache opt-in

### Traits

- `Cacheable`: Trait providing default caching functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.