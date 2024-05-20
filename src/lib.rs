//! # Serde JSON Borrowed
//!
//! Parses JSON into [`serde_json_borrow::Value<'ctx>`](Value) from `&'ctx str`.
//!
//! The default [serde_json](https://github.com/serde-rs/json) parses into an owned `serde_json::Value`.
//! In cases where the DOM representation is just an intermediate struct, parsing into owned
//! `serde_json::Value` can cause a lot of overhead. [`serde_json_borrow::Value<'ctx>`](Value)
//! borrows the `Strings` instead.
//!
//! Additionally it pushes the (key,value) for JSON objects into a `Vec` instead of putting the
//! values into a `BTreeMap`. Access works via an iterator, which has the same API when iterating
//! the `BTreeMap`.
//!
//! The primary benefit of using `serde_json_borrow` is a higher JSON _deserialization performance_
//! due to less allocations. By borrowing a DOM, the library ensures that no additional memory is
//! allocated for `Strings`, that contain no JSON escape codes.
//!
//! ## OwnedValue
//! You can take advantage of [`OwnedValue`](crate::OwnedValue) to parse a `String` containing
//! unparsed `JSON` into a `Value` without having to worry about lifetimes,
//! as [`OwnedValue`](crate::OwnedValue) will take ownership of the `String` and reference slices of
//! it, rather than making copies.
//!
//! # Limitations
//! Keys in objects are not allowed to have any JSON escaping characters. So if your keys contain any control characters <https://www.json.org/json-en.html>, this crate will not work for you.
//! List of _unsupported_ characters in keys.
//!
//! ```text
//! \" represents the quotation mark character (U+0022).
//! \\ represents the reverse solidus character (U+005C).
//! \/ represents the solidus character (U+002F).
//! \b represents the backspace character (U+0008).
//! \f represents the form feed character (U+000C).
//! \n represents the line feed character (U+000A).
//! \r represents the carriage return character (U+000D).
//! \t represents the character tabulation character (U+0009).
//! ```
//! # Usage
//! ```rust
//! use std::io;
//! use serde_json_borrow::Value;
//! fn main() -> io::Result<()> {
//!     let data = r#"{"bool": true, "key": "123"}"#;
//!     let value: Value = serde_json::from_str(&data)?;
//!     assert_eq!(value.get("bool"), &Value::Bool(true));
//!     assert_eq!(value.get("key"), &Value::Str("123".into()));
//!     Ok(())
//! }
//! ```
//! # Performance
//! Performance gain depends on how many allocations can be avoided. It also depends on the
//! allocator used.
//! The [benchmarks](https://github.com/pseitz/serde_json_borrow#benchmark) in the github repository show around **`1.8x`** speedup, although they don't account
//! for that in practice it won't be a simple consecutive alloc json, dealloc json. There will be
//! other allocations in between.
//!
//! On a hadoop file system log data set benchmark, I get _714Mb/s_ JSON deserialization throughput
//! on my machine.

mod de;
mod index;
mod object_vec;
mod owned;
mod ser;
mod value;

pub use object_vec::ObjectAsVec;
pub use owned::OwnedValue;
pub use value::Value;
