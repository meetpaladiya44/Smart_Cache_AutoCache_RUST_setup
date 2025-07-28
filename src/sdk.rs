extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{String, ToString};
use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256, evm};
use stylus_sdk::alloy_sol_types::sol;

// Event definition for AutoCacheOptIn (optional - for manual opt-in)
sol! {
    event AutoCacheOptIn(address indexed contract_addr);
}
 
// Define the contract with storage and entrypoint
#[storage]
pub struct MyCacheableContract {
    counter: StorageU256,
}

// External functions
#[public]
impl MyCacheableContract {
    /// 🚨 REQUIRED: SmartCache Detection Function
    /// This is the ONLY function needed for SmartCache integration
    pub fn is_cacheable(&self) -> bool {
        true  // Return true to enable SmartCache for this contract
    }

    /// Optional: Manual opt-in function (emits event)
    pub fn opt_in_to_cache(&mut self) {
        evm::log(AutoCacheOptIn {
            contract_addr: self.vm().contract_address(),
        });
    }

    /// Increment the counter by 1 and return the new value
    pub fn increment_counter(&mut self) -> U256 {
        let current = self.counter.get();
        let new_value = current + U256::from(1);
        self.counter.set(new_value);
        new_value
    }

    /// Simulated expensive function to demonstrate caching benefits
    pub fn expensive_logic(&self, input: u64) -> u64 {
        let mut result = input;
        for _ in 0..5000 {
            result = result.wrapping_mul(123456789).wrapping_add(987654321);
        }
        result
    }

    /// Simple helper to verify contract functionality
    pub fn ping(&self) -> String {
        "pong".to_string()
    }
}

// Helper function that can be used by other contracts
pub fn is_contract_cacheable() -> bool {
    true
} 