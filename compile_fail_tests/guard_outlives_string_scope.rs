//! This test demonstrates that a guard cannot outlive the JSON string it references.
//! This file should fail to compile with an error like:
//! EXPECT: error[E0597]: `json_string` does not live long enough

use serde_json_borrow::ReusableMap;

fn main() {
    let mut deserializer = ReusableMap::new();
    let guard;

    {
        // Create a JSON string with a limited scope
        let json_string = r#"{"temporary":"value"}"#.to_string();

        // This should fail to compile because the guard would outlive json_string
        guard = deserializer.deserialize(&json_string).unwrap();

        // The guard borrows from json_string, but json_string will be dropped
        // at the end of this block, while guard would live longer
    }

    // This would be a use-after-free if the compiler allowed it
    assert_eq!(guard.len(), 1);
}
