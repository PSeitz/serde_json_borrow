

# serde_json_borrow

Parse json into DOM on borrowed data.

`serde_json` parses into an owned `serde_json::Value`.

In cases where the DOM representation is just an intermediate struct, parsing into owned `String` can cause a lot of overhead.
`serde_json_borrow::Value<'ctx>` references the strings instead. Instead of putting the values into a `BTreeMap` it pushed the values into a `Vec`.
Access works via an iterator, which has the same API when iterating the `BTreeMap`.


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

Benchmark are not extensive yet, but they look very promising.

```

Running benches/crit_bench.rs (/home/pascal/cargo_target_dir/release/deps/crit_bench-fd2d661e0b4255c5)
flat-json-to-doc/serde-json-owned
                        time:   [352.60 µs 353.40 µs 354.26 µs]
                        thrpt:  [236.01 MiB/s 236.59 MiB/s 237.12 MiB/s]
flat-json-to-doc/serde-json-borrowed
                        time:   [175.53 µs 175.72 µs 175.93 µs]
                        thrpt:  [475.23 MiB/s 475.81 MiB/s 476.34 MiB/s]

```

