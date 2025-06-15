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

Note: `OwnedValue` does not implement `Deserialize`.

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

Access benchmarks are done to show the relative overhead of accessing data on the resulting `Value` DOM, which
is insignificant compared to the parsing time here.
Since `serde_json_borrow` deserializes into a `Vec` instead of a `BTreeMap`.

In the benchmarks below it consistently outperforms `serde_json` and `simd-json`.
Benchmarks are done on a AMD Ryzen 7 9800X3D on 6.14.6-2-MANJARO.


```
parse
simple_json
serde_json                                           Avg: 297.30 MB/s     Median: 296.71 MB/s     [293.91 MB/s .. 312.10 MB/s]    
serde_json + access by key                           Avg: 297.43 MB/s     Median: 296.54 MB/s     [285.99 MB/s .. 306.42 MB/s]    
serde_json_borrow::OwnedValue                        Avg: 552.02 MB/s     Median: 552.80 MB/s     [538.45 MB/s .. 555.48 MB/s]    
serde_json_borrow::OwnedValue + access by key        Avg: 535.23 MB/s     Median: 535.60 MB/s     [524.21 MB/s .. 537.36 MB/s]    
SIMD_json_borrow                                     Avg: 296.12 MB/s     Median: 296.68 MB/s     [289.54 MB/s .. 297.27 MB/s]    
hdfs
serde_json                                           Avg: 688.77 MB/s     Median: 690.78 MB/s     [660.10 MB/s .. 698.89 MB/s]    
serde_json + access by key                           Avg: 675.24 MB/s     Median: 675.85 MB/s     [661.28 MB/s .. 683.70 MB/s]    
serde_json_borrow::OwnedValue                        Avg: 1.1158 GB/s     Median: 1.1175 GB/s     [1.0847 GB/s .. 1.1301 GB/s]    
serde_json_borrow::OwnedValue + access by key        Avg: 1.1044 GB/s     Median: 1.1085 GB/s     [1.0040 GB/s .. 1.1262 GB/s]    
SIMD_json_borrow                                     Avg: 687.72 MB/s     Median: 688.76 MB/s     [663.14 MB/s .. 700.18 MB/s]    
hdfs_with_array
serde_json                                           Avg: 468.18 MB/s     Median: 466.53 MB/s     [455.06 MB/s .. 486.57 MB/s]    
serde_json + access by key                           Avg: 478.47 MB/s     Median: 478.35 MB/s     [467.25 MB/s .. 494.33 MB/s]    
serde_json_borrow::OwnedValue                        Avg: 813.81 MB/s     Median: 816.95 MB/s     [770.63 MB/s .. 825.62 MB/s]    
serde_json_borrow::OwnedValue + access by key        Avg: 821.20 MB/s     Median: 824.86 MB/s     [788.96 MB/s .. 833.78 MB/s]    
SIMD_json_borrow                                     Avg: 536.68 MB/s     Median: 538.81 MB/s     [517.16 MB/s .. 545.30 MB/s]    
wiki
serde_json                                           Avg: 1.3004 GB/s     Median: 1.3014 GB/s     [1.2670 GB/s .. 1.3182 GB/s]    
serde_json + access by key                           Avg: 1.3521 GB/s     Median: 1.3531 GB/s     [1.3180 GB/s .. 1.3678 GB/s]    
serde_json_borrow::OwnedValue                        Avg: 1.5089 GB/s     Median: 1.5072 GB/s     [1.4898 GB/s .. 1.5289 GB/s]    
serde_json_borrow::OwnedValue + access by key        Avg: 1.5656 GB/s     Median: 1.5638 GB/s     [1.5393 GB/s .. 1.5945 GB/s]    
SIMD_json_borrow                                     Avg: 1.4824 GB/s     Median: 1.4838 GB/s     [1.4146 GB/s .. 1.5250 GB/s]    
gh-archive
serde_json                                           Avg: 451.29 MB/s     Median: 451.74 MB/s     [439.02 MB/s .. 455.40 MB/s]    
serde_json + access by key                           Avg: 453.74 MB/s     Median: 454.52 MB/s     [444.96 MB/s .. 457.46 MB/s]    
serde_json_borrow::OwnedValue                        Avg: 1.1181 GB/s     Median: 1.1236 GB/s     [1.0584 GB/s .. 1.1467 GB/s]    
serde_json_borrow::OwnedValue + access by key        Avg: 1.1361 GB/s     Median: 1.1416 GB/s     [1.0744 GB/s .. 1.1611 GB/s]    
SIMD_json_borrow                                     Avg: 992.99 MB/s     Median: 995.00 MB/s     [956.99 MB/s .. 1.0086 GB/s]    
access
simple_json
serde_json access               Avg: 8.9616 GB/s     Median: 8.9910 GB/s     [8.5910 GB/s .. 9.0113 GB/s]    Output: 7_002    
serde_json_borrow access        Avg: 30.654 GB/s     Median: 30.859 GB/s     [29.639 GB/s .. 31.056 GB/s]    Output: 7_002    
gh-archive
serde_json access               Avg: 15.686 GB/s     Median: 15.729 GB/s     [15.137 GB/s .. 15.843 GB/s]    Output: 231_243    
serde_json_borrow access        Avg: 35.535 GB/s     Median: 35.575 GB/s     [34.149 GB/s .. 37.598 GB/s]    Output: 231_243    

```

# TODO 
Instead of parsing a JSON object into a `Vec`, a `BTreeMap` could be enabled via a feature flag.

# Mutability
`OwnedValue` is immutable by design.
If you need to mutate the `Value` you can convert it to `serde_json::Value`.

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
