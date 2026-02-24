//! This test demonstrates that a guard cannot be returned from a function
//! where the JSON string is local to that function.
//! This file should fail to compile with an error about lifetimes not matching.
//! EXPECT: error[E0515]: cannot return value referencing local variable `json_string`

use serde_json_borrow::{BorrowedMap, ReusableMap};

// This function tries to return a guard that references a local JSON string
fn create_guard<'d>(deserializer: &'d mut ReusableMap) -> BorrowedMap<'static, 'd> {
    // Local JSON string that will be dropped when the function returns
    let json_string = r#"{"escape":"attempt"}"#.to_string();

    // This should fail to compile - cannot convert BorrowedMap<'_, 'd> to BorrowedMap<'static, 'd>
    // because that would allow the guard to outlive the json_string
    deserializer.deserialize(&json_string).unwrap()
}

fn main() {
    let mut deserializer = ReusableMap::new();

    // Try to get a guard with an invalid 'static lifetime for the JSON string
    let guard = create_guard(&mut deserializer);

    // This would be a use-after-free if the compiler allowed it
    assert_eq!(guard.len(), 1);
}
