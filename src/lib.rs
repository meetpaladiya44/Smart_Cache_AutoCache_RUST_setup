#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{String, ToString};
use stylus_sdk::{alloy_primitives::U256, prelude::*};

mod sdk;
use crate::sdk::{is_contract_cacheable};

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

    /// Gets the number from storage.
    pub fn numbers(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_numbers(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_numbers(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment_counter(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    /// Adds the wei value from msg_value to the number in storage.
    #[payable]
    pub fn add_from_msg_value(&mut self) {
        let number = self.number.get();
        self.set_number(number + self.vm().msg_value());
    }

    pub fn ping(&self) -> String {
        "pong".to_string()
    }

    /// Decrements `number` and updates its value in storage.
    pub fn decrement(&mut self) {
        let number = self.number.get();
        if number > U256::ZERO {
            self.set_number(number - U256::from(1));
        }
    }

    /// Subtracts a number from the current value in storage.
    pub fn subtract_numbers(&mut self, sub_number: U256) {
        let number = self.number.get();
        if number >= sub_number {
            self.set_number(number - sub_number);
        }
    }

    /// Divides the current number by a given divisor.
    pub fn divide_numbers(&mut self, divisor: U256) {
        if divisor > U256::ZERO {
            let number = self.number.get();
            self.set_number(number / divisor);
        }
    }

    /// Resets the number to zero.
    pub fn reset(&mut self) {
        self.number.set(U256::ZERO);
    }

    /// Squares the current number.
    pub fn square(&mut self) {
        let number = self.number.get();
        self.set_number(number * number);
    }

    /// Raises the current number to the specified power.
    pub fn power_numbers(&mut self, exponent: U256) {
        let number = self.number.get();
        self.set_number(number.pow(exponent));
    }

    /// Returns double the current number without modifying storage.
    pub fn get_double(&self) -> U256 {
        self.number.get() * U256::from(2)
    }

    /// Returns triple the current number without modifying storage.
    pub fn get_triple(&self) -> U256 {
        self.number.get() * U256::from(3)
    }

    /// Checks if the current number is even.
    pub fn is_even(&self) -> bool {
        self.number.get() % U256::from(2) == U256::ZERO
    }

    /// Checks if the current number is odd.
    pub fn is_odd(&self) -> bool {
        !self.is_even()
    }

    /// Checks if the current number is zero.
    pub fn is_zero(&self) -> bool {
        self.number.get() == U256::ZERO
    }

    /// Sets the number to the maximum of current value and provided value.
    pub fn set_maximum_numbers(&mut self, compare_number: U256) {
        let current = self.number.get();
        if compare_number > current {
            self.set_number(compare_number);
        }
    }

    /// Sets the number to the minimum of current value and provided value.
    pub fn set_minimum_numbers(&mut self, compare_number: U256) {
        let current = self.number.get();
        if compare_number < current {
            self.set_number(compare_number);
        }
    }

    /// Increments the number by a specified amount.
    pub fn increment_by_numbers(&mut self, amount: U256) {
        self.add_numbers(amount);
    }

    /// Returns the current number modulo the given divisor.
    pub fn modulo(&self, divisor: U256) -> U256 {
        if divisor > U256::ZERO {
            self.number.get() % divisor
        } else {
            U256::ZERO
        }
    }

    /// Swaps the current number with a new number and returns the old value.
    pub fn swap_numbers(&mut self, new_number: U256) -> U256 {
        let old_number = self.number.get();
        self.set_number(new_number);
        old_number
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counter_numbers() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Counter::from(&vm);

        assert_eq!(U256::ZERO, contract.numbers());

        contract.increment_counter();
        assert_eq!(U256::from(1), contract.numbers());

        contract.add_numbers(U256::from(3));
        assert_eq!(U256::from(4), contract.numbers());

        contract.mul_numbers(U256::from(2));
        assert_eq!(U256::from(8), contract.numbers());

        contract.set_number(U256::from(100));
        assert_eq!(U256::from(100), contract.numbers());

        // Override the msg value for future contract method invocations.
        vm.set_value(U256::from(2));

        contract.add_from_msg_value();
        assert_eq!(U256::from(102), contract.numbers());
    }

    #[test]
    fn test_is_cacheable() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let contract = Counter::from(&vm);

        assert_eq!(true, contract.is_cacheable());
    }
}