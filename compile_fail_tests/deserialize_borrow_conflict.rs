//! This test demonstrates that the deserializer cannot be used while a guard exists.
//! This file should fail to compile because the deserializer is already borrowed by the first guard.
//! EXPECT: error[E0499]: cannot borrow `deserializer` as mutable more than once at a time

use serde_json_borrow::ReusableMap;

fn main() {
    let mut deserializer = ReusableMap::new();

    // First JSON string
    let json_str1 = r#"{"first":"value"}"#;

    // Get a guard from the deserializer
    let guard1 = deserializer.deserialize(json_str1).unwrap();

    // Try to use the deserializer again while guard1 exists
    // This should fail to compile because the deserializer is already mutably borrowed
    let json_str2 = r#"{"second":"value"}"#;
    let guard2 = deserializer.deserialize(json_str2).unwrap(); // Should fail with borrow error

    // Use both guards to ensure the compiler doesn't optimize away
    println!("First guard length: {}", guard1.len());
    println!("Second guard length: {}", guard2.len());
}
