# Compile-Fail Tests

This directory contains tests that are designed to fail at compile time. These tests verify that our lifetime safety guarantees in the `deser` module are properly enforced by the Rust compiler.

## Purpose

These tests ensure that:

1. A guard cannot outlive the JSON string it references (`guard_outlives_string_scope`)
2. A guard cannot be returned from a function where the JSON string is local (`guard_return_from_function`)
3. References cannot be leaked from a guard to outlive their source (`leak_references_from_guard`)
4. A deserializer cannot be used while a guard exists (`safety_tests/deserializer_borrow_conflict`)
5. A guard's internal map cannot be borrowed mutably multiple times (`safety_tests/guard_multiple_borrows`)
6. Guards cannot be sent between threads if they contain references to thread-local data (`safety_tests/guard_thread_safety`)
7. The lifetime of the map is properly tied to the JSON string's lifetime (`safety_tests/map_lifetime_soundness`)

## How to Run

These tests are meant to fail at compile time, so you can verify them by trying to build each test individually:

```bash
# Test that a guard cannot outlive its string reference
cd guard_outlives_string_scope
cargo build
# Should fail with: error[E0597]: `json_string` does not live long enough

# Test that a guard cannot be returned from a function with local string
cd ../guard_return_from_function
cargo build
# Should fail with lifetime errors

# Test that references cannot be leaked from a guard
cd ../leak_references_from_guard
cargo build
# Should fail with lifetime errors

# Additional safety tests
cd ../safety_tests
cargo check --bin deserializer_borrow_conflict  # Should fail with "cannot borrow as mutable"
cargo check --bin guard_multiple_borrows        # Should fail with "cannot borrow as mutable more than once"
cargo check --bin guard_thread_safety           # Should fail with "cannot be sent between threads safely"
cargo check --bin map_lifetime_soundness        # Should fail with "does not live long enough"
```

## Expected Errors

Each test demonstrates a different aspect of lifetime safety:

- `guard_outlives_string_scope`: Should fail with `error[E0597]: 'json_string' does not live long enough`
- `guard_return_from_function`: Should fail with errors about lifetimes not matching
- `leak_references_from_guard`: Should fail with `error[E0597]: 'json_string' does not live long enough`
- `deserializer_borrow_conflict`: Should fail with "cannot borrow as mutable" error
- `guard_multiple_borrows`: Should fail with "cannot borrow as mutable more than once" error
- `guard_thread_safety`: Should fail with "cannot be sent between threads safely" error
- `map_lifetime_soundness`: Should fail with "does not live long enough" error

## Why This Matters

These tests are crucial for verifying that our guard pattern correctly prevents use-after-free bugs by enforcing compile-time lifetime guarantees. If any of these tests were to compile successfully, it would indicate a flaw in our safety guarantees.

The safety of our deserialization approach depends on these lifetime constraints being properly enforced by the compiler. These tests give us confidence that the guard pattern is working as intended to prevent memory safety issues.

## Additional Runtime Tests

In addition to these compile-fail tests, there is a runtime test `test_map_cleared_on_drop` in the main test suite that verifies that the map is properly cleared when the guard is dropped, ensuring no dangling references remain.