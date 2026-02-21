#![deny(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_imports,
    unused_qualifications,
    missing_docs
)]

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
//! values into a `BTreeMap`. Access works via `ObjectAsVec`, which provides the same API
//! as `BTreeMap`.
//!
//! The primary benefit of using `serde_json_borrow` is a higher JSON _deserialization performance_
//! due to less allocations. By borrowing a DOM, the library ensures that no additional memory is
//! allocated for `Strings`, that contain no JSON escape codes.
//!
//! # Usage
//! ```rust
//! use std::io;
//! fn main() -> io::Result<()> {
//!     let data: &str = r#"{"bool": true, "key": "123"}"#;
//!     // Note that serde_json_borrow::Value<'ctx> is tied to the lifetime of data.
//!     let value: serde_json_borrow::Value = serde_json::from_str(&data)?;
//!     assert_eq!(value.get("bool"), Some(&serde_json_borrow::Value::Bool(true)));
//!     assert_eq!(value.get("key"), Some(&serde_json_borrow::Value::Str("123".into())));
//!     // Using OwnedValue will take ownership of the String.
//!     let value: serde_json_borrow::OwnedValue = serde_json_borrow::OwnedValue::from_str(&data)?;
//!     assert_eq!(value.get("bool"), Some(&serde_json_borrow::Value::Bool(true)));
//!     assert_eq!(value.get("key"), Some(&serde_json_borrow::Value::Str("123".into())));
//!     Ok(())
//! }
//! ```
//!
//! ## OwnedValue
//! You can take advantage of [`OwnedValue`] to parse a `String` containing
//! unparsed `JSON` into a `Value` without having to worry about lifetimes,
//! as [`OwnedValue`] will take ownership of the `String` and reference slices of
//! it, rather than making copies.
//!
//! # Limitations
//! The feature flag `cowkeys` uses `Cow<str>` instead of `&str` as keys in objects. This enables
//! support for escaped data in keys. Without the `cowkeys` feature flag `&str` is used, which does
//! not allow any JSON escaping characters in keys.
//!
//! List of _unsupported_ characters (<https://www.json.org/json-en.html>) in object keys without `cowkeys` feature flag.
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
//! # Performance
//! Performance gain depends on how many allocations can be avoided, and how many objects there are,
//! as deserializing into a vec is significantly faster.
//!
//! The [benchmarks](https://github.com/pseitz/serde_json_borrow#benchmark) in the github repository show around **`1.8x`** speedup, although they don't account
//! for that in practice it won't be a simple consecutive alloc json, dealloc json. There will be
//! other allocations in between.
//!
//! On a hadoop file system log data set benchmark, I get _714Mb/s_ JSON deserialization throughput
//! on my machine.

mod de;
mod deserializer;
mod index;
mod num;
mod object_vec;
mod ownedvalue;
mod ser;
mod value;

#[cfg(feature = "cowkeys")]
mod cowstr;

pub use num::Number;
pub use object_vec::{KeyStrType, ObjectAsVec, ObjectAsVec as Map, ObjectEntry};
pub use ownedvalue::OwnedValue;
pub use value::Value;
