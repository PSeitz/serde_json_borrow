//! This test demonstrates that a BorrowedMap cannot be sent across thread boundaries
//! when it contains references to stack data.
//!
//! This file should fail to compile because we're trying to move a guard with
//! references to stack variables into a new thread.
//! EXPECT: error[E0597]: `json_string` does not live long enough
//! EXPECT: argument requires that `json_string` is borrowed for `'static`

use std::thread;

use serde_json_borrow::ReusableMap;

fn main() {
    // Create a JSON string on the stack
    let json_string = r#"{"key":"value"}"#.to_string();

    // Create a deserializer and guard
    let mut deserializer = ReusableMap::new();
    let guard = deserializer.deserialize(&json_string).unwrap();

    // Attempt to move the guard to a new thread
    // This should fail to compile because the guard contains references
    // to stack data (json_string and deserializer) that won't be valid
    // in the new thread.
    let handle = thread::spawn(move || {
        // Try to use the guard in the new thread
        println!("Guard in thread: {} elements", guard.len());
    });

    handle.join().unwrap();
}
