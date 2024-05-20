[![Crates.io](https://img.shields.io/crates/v/serde_json_borrow.svg)](https://crates.io/crates/serde_json_borrow)
 [![Docs](https://docs.rs/serde_json_borrow/badge.svg)](https://docs.rs/crate/serde_json_borrow/)
 
# Serde JSON Borrow

Up to 2x faster JSON parsing for [ndjson](http://ndjson.org/) type use cases.

`serde_json_borrow` deserializes JSON from `&'ctx str` into `serde_json_borrow::Value<'ctx>` DOM, by trying to reference the original bytes, instead of copying them into `Strings`.

In contrast the default [serde_json](https://github.com/serde-rs/json) parses into an owned `serde_json::Value`. Every `String` encountered is getting copied and 
therefore allocated. That's great for ergnomonics, but not great for performance.
Especially in cases where the DOM representation is just an intermediate struct.

To get a little bit more performance, `serde_json_borrow` pushes the (key,values) for JSON objects into a `Vec` instead of using a `BTreeMap`. Access works via
an iterator, which has the same API when iterating the `BTreeMap`.

## OwnedValue
You can take advantage of `OwnedValue` to parse a `String` containing unparsed `JSON` into a `Value` without having to worry about lifetimes,
as `OwnedValue` will take ownership of the `String` and reference slices of it, rather than making copies.

# Limitations
Keys in objects are not allowed to have any JSON escaping characters. So if your keys contain any control characters (https://www.json.org/json-en.html), this crate will not work for you.
List of _unsupported_ characters in keys.

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
