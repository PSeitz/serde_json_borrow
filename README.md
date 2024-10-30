[![Crates.io](https://img.shields.io/crates/v/serde_json_borrow.svg)](https://crates.io/crates/serde_json_borrow)
 [![Docs](https://docs.rs/serde_json_borrow/badge.svg)](https://docs.rs/crate/serde_json_borrow/)
 
# Serde JSON Borrow

Up to 2x faster JSON parsing for NDJSON (Newline Delimited JSON format) type use cases.

`serde_json_borrow` deserializes JSON from `&'ctx str` into `serde_json_borrow::Value<'ctx>` DOM, by trying to reference the original bytes, instead of copying them into `Strings`.

In contrast the default [serde_json](https://github.com/serde-rs/json) parses into an owned `serde_json::Value`. Every `String` encountered is getting copied and 
therefore allocated. That's great for ergonomonics, but not great for performance.
Especially in cases where the DOM representation is just an intermediate struct.

To get a little bit more performance, `serde_json_borrow` pushes the (key,values) for JSON objects into a `Vec` instead of using a `BTreeMap`. Access works via
an iterator, which has the same API when iterating the `BTreeMap`.

## OwnedValue
You can take advantage of `OwnedValue` to parse a `String` containing unparsed `JSON` into a `Value` without having to worry about lifetimes,
as `OwnedValue` will take ownership of the `String` and reference slices of it, rather than making copies.

# Limitations
The feature flag `cowkeys` uses `Cow<str>` instead of `&str` as keys in objects. This enables support for escaped data in keys.
Without the `cowkeys` feature flag `&str` is used, which does not allow any JSON escaping characters in keys.

List of _unsupported_ characters (https://www.json.org/json-en.html) in keys without `cowkeys` feature flag.

```
\" represents the quotation mark character (U+0022).
\\ represents the reverse solidus character (U+005C).
\/ represents the solidus character (U+002F).
\b represents the backspace character (U+0008).
\f represents the form feed character (U+000C).
\n represents the line feed character (U+000A).
\r represents the carriage return character (U+000D).
\t represents the character tabulation character (U+0009).
```

# Benchmark

`cargo bench`

* simple_json -> flat object with some keys
* hdfs -> log
* wiki -> few keys with large text body 
* gh-archive -> highly nested object
```
simple_json
serde_json               Avg: 139.29 MiB/s    Median: 139.53 MiB/s    [134.51 MiB/s .. 140.45 MiB/s]    
serde_json_borrow        Avg: 210.33 MiB/s    Median: 209.66 MiB/s    [204.08 MiB/s .. 214.28 MiB/s]    
SIMD_json_borrow         Avg: 140.36 MiB/s    Median: 140.44 MiB/s    [138.96 MiB/s .. 141.75 MiB/s]    
hdfs
serde_json               Avg: 284.64 MiB/s    Median: 284.60 MiB/s    [280.98 MiB/s .. 286.46 MiB/s]    
serde_json_borrow        Avg: 372.99 MiB/s    Median: 371.75 MiB/s    [365.97 MiB/s .. 379.96 MiB/s]    
SIMD_json_borrow         Avg: 294.41 MiB/s    Median: 294.96 MiB/s    [287.76 MiB/s .. 296.96 MiB/s]    
hdfs_with_array
serde_json               Avg: 194.50 MiB/s    Median: 200.41 MiB/s    [155.44 MiB/s .. 211.49 MiB/s]    
serde_json_borrow        Avg: 275.01 MiB/s    Median: 282.74 MiB/s    [208.35 MiB/s .. 289.78 MiB/s]    
SIMD_json_borrow         Avg: 206.34 MiB/s    Median: 210.52 MiB/s    [180.99 MiB/s .. 220.30 MiB/s]    
wiki
serde_json               Avg: 439.95 MiB/s    Median: 441.28 MiB/s    [429.97 MiB/s .. 444.82 MiB/s]    
serde_json_borrow        Avg: 484.74 MiB/s    Median: 485.29 MiB/s    [471.38 MiB/s .. 489.16 MiB/s]    
SIMD_json_borrow         Avg: 576.57 MiB/s    Median: 578.11 MiB/s    [554.03 MiB/s .. 586.18 MiB/s]    
gh-archive
serde_json               Avg: 176.21 MiB/s    Median: 176.37 MiB/s    [172.52 MiB/s .. 177.78 MiB/s]    
serde_json_borrow        Avg: 363.58 MiB/s    Median: 364.02 MiB/s    [355.28 MiB/s .. 374.10 MiB/s]    
SIMD_json_borrow         Avg: 383.66 MiB/s    Median: 386.94 MiB/s    [363.80 MiB/s .. 400.25 MiB/s]    

```

# TODO 
Instead of parsing a JSON object into a `Vec`, a `BTreeMap` could be enabled via a feature flag.

# Mutability
`OwnedValue` is immutable by design.
If you need to mutate the `Value` you can convert it to `serde_json::Value`.

## Example
Here is an example why mutability won't work:

https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bb0b919acc8930e71bdefdfc6a6d5240
```rust
use std::io;

use std::borrow::Cow;


/// Parses a `String` into `Value`, by taking ownership of `String` and reference slices from it in
/// contrast to copying the contents.
///
/// This is done to mitigate lifetime issues.
pub struct OwnedValue {
    /// Keep owned data, to be able to safely reference it from Value<'static>
    _data: String,
    value: Vec<Cow<'static, str>>,
}

impl OwnedValue {
    /// Takes ownership of a `String` and parses it into a DOM.
    pub fn parse_from(data: String) -> io::Result<Self> {
        let value = vec![Cow::from(data.as_str())];
        let value = unsafe { extend_lifetime(value) };
        Ok(Self { _data: data, value })
    }

    /// Returns the `Value` reference.
    pub fn get_value<'a>(&'a self) -> &'a Vec<Cow<'a, str>> {
        &self.value
    }
    /// This cast will break the borrow checker
    pub fn get_value_mut<'a>(&'a mut self) -> &'a mut Vec<Cow<'a, str>> {
        unsafe{std::mem::transmute::<&mut Vec<Cow<'static, str>>, &mut Vec<Cow<'a, str>>>(&mut self.value)}
    }
}

unsafe fn extend_lifetime<'b>(r: Vec<Cow<'b, str>>) -> Vec<Cow<'static, str>> {
    std::mem::transmute::<Vec<Cow<'b, str>>, Vec<Cow<'static, str>>>(r)
}

fn main() {
    let mut v1 = OwnedValue::parse_from(String::from("oop")).unwrap();
    let mut v2 = OwnedValue::parse_from(String::from("oop")).unwrap();
    let oop = v1.get_value().last().unwrap().clone();
    v2.get_value_mut().push(oop);
    drop(v1);
    let oop = v2.get_value_mut().pop().unwrap();
    println!("oop: '{oop}'");
}
```
