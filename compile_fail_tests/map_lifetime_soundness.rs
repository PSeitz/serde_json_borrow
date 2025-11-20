//! This test demonstrates that the lifetime of the map is properly tied to the JSON string.
//! This file should fail to compile because we're trying to store a reference from the map
//! that would outlive the JSON string it references.
//! EXPECT: error[E0597]: `json_string` does not live long enough
//! EXPECT: error[E0597]: `guard` does not live long enough

use serde_json_borrow::{ReusableMap, Value};

fn main() {
    let mut deserializer = ReusableMap::new();
    let stored_ref: &str;

    {
        // Create a JSON string with a limited scope
        let json_string = r#"{"key":"value"}"#.to_string();
        let guard = deserializer.deserialize(&json_string).unwrap();

        // Try to store a reference from the map that would outlive the JSON string
        if let Some(Value::Str(val)) = guard.get("key") {
            // This should fail to compile - cannot assign a reference with lifetime
            // tied to json_string to a variable that outlives json_string
            stored_ref = val;
        } else {
            unreachable!();
        }
    }

    // Try to use the stored reference after the JSON string is dropped
    // This would be a use-after-free if the compiler allowed it
    println!("Stored ref: {}", stored_ref);
}
