extern crate alloc;

/// Returns whether a contract is cacheable.
/// This function can be imported and used directly in other contracts.
pub fn is_contract_cacheable() -> bool {
    true
}

/// To emit the `AutoCacheOptIn` event in your contract, include the following event definition:
///
/// ```rust
/// use alloy_sol_types::sol;
///
/// sol! {
///     event AutoCacheOptIn(address indexed contract_addr);
/// }
/// ```
///
/// Then, in your contract methods, emit the event using `stylus-sdk`'s event emission mechanism.
/// For example (syntax may vary based on `stylus-sdk`):
///
/// ```rust
/// #[public]
/// impl YourContract {
///     pub fn opt_in(&mut self) {
///         // Emit the event with the contract's address
///         self.emit(AutoCacheOptIn { contract_addr: self.address() });
///     }
/// }
/// ```
///
/// Make sure to include `alloy-sol-types` as a dependency in your `Cargo.toml`:
/// ```toml
/// [dependencies]
/// alloy-sol-types = "0.7.7"
/// stylus-sdk = "0.4.3"
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_contract_cacheable() {
        assert!(is_contract_cacheable());
    }
}