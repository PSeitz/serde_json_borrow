0.7.1 (2024-11-02)
==================
strip extra iteration when initialising ObjectAsVec https://github.com/PSeitz/serde_json_borrow/pull/29 (Thanks @meskill) 

0.7.0 (2024-10-28)
==================
impl `Deserializer` for `Value`. This enables deserialization into other types.

0.6.0 (2024-08-28)
==================
improve returned lifetimes https://github.com/PSeitz/serde_json_borrow/pull/19 (Thanks @meskill) 

0.5.0 (2024-05-24)
==================
add `cowkeys` featureflag 
`cowkeys` uses `Cow<str>` instead of `&str` as keys in objects. This enables support for escaped data in keys.

0.4.7 (2024-05-23)
==================
add `From` methods for type like u64 to easily convert into `Value<'a>`

0.4.6 (2024-05-23)
==================
add `Clone, Debug, Eq, PartialEq, Hash` to OwnedValue 
add `Hash` to Value 
implement numeric visitor, visit_u8, i8 etc.

0.4.5 (2024-05-22)
==================
add `get_mut`, `insert` to serde_json_borrow::Map
add `insert_or_get_mut` to serde_json_borrow::Map as `entry()` alternative
add From<&Value> for `serde_json::Value`
add From<serde_json_borrow::Map> for serde_json::Map

0.4.4 (2024-05-22)
==================
Export `ObjectAsVec` as `serde_json_borrow::Map` to easy migration from `serde_json::Map`
Impl `Default` for `serde_json_borrow::Map`

0.4.3 (2024-05-22)
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
