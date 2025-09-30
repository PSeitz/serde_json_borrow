//! This test demonstrates that references cannot be leaked from a guard.
//! This file should fail to compile with an error about lifetimes not matching.
//! EXPECT: error[E0597]: `json_string` does not live long enough
//! EXPECT: error[E0597]: `guard` does not live long enough

use serde_json_borrow::ReusableMap;
use serde_json_borrow::Value;

fn main() {
    let mut deserializer = ReusableMap::new();
    let leaked_ref: &str;

    {
        let json_string = r#"{"name":"test"}"#.to_string();
        let guard = deserializer.deserialize(&json_string).unwrap();

        // Try to extract and leak a reference from the guard
        if let Some(Value::Str(name)) = guard.get("name") {
            // This should fail to compile - cannot assign a reference with lifetime
            // tied to json_string to a variable that outlives json_string
            leaked_ref = name;
        } else {
            unreachable!();
        }
    }

    // This would be a use-after-free if the compiler allowed it
    println!("Leaked reference: {}", leaked_ref);
}
