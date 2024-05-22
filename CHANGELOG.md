0.4.5 (2024-05-24)
==================
add `get_mut`, `insert` to serde_json_borrow::Map
add `insert_or_get_mut` to serde_json_borrow::Map as `entry()` alternative
add From<&Value> for `serde_json::Value`
add From<serde_json_borrow::Map> for serde_json::Map

0.4.4 (2024-05-24)
==================
Export `ObjectAsVec` as `serde_json_borrow::Map` to easy migration from `serde_json::Map`
Impl `Default` for `serde_json_borrow::Map`

0.4.3 (2024-05-24)
==================
add `From<Vec`> for `ObjectAsVec`

0.4.2 (2024-05-20)
==================
* Add `OwnedValue::from_slice`, `OwnedValue::from_str` and `OneValue::from_string`
* Add `Deref<Target=Value>` for `OwnedValue` https://github.com/PSeitz/serde_json_borrow/pull/16

0.4.1 (2024-05-20)
==================
* Implement `Display` on serde_json_borrow::Value https://github.com/PSeitz/serde_json_borrow/pull/15

0.4 (2024-05-20)
==================
* Add Object access methods on Objects via `ObjectAsVec` wrapper https://github.com/PSeitz/serde_json_borrow/pull/12

json objects are backed by `Vec` in `serde_json_borrow`, but it was missing the same methods like `BtreeMap`, e.g. `.get` `get_key_value`.
ObjectAsVec provides these methods.

0.3 (2023-09-19)
==================
* Add as_object, as_array methods https://github.com/PSeitz/serde_json_borrow/pull/8

0.2 (2023-07-07)
==================
* Add serialization https://github.com/PSeitz/serde_json_borrow/pull/6 (Thanks @dtolnay) 
