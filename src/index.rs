use super::Value;

/// A type that can be used to index into a `serde_json_borrow::Value`.
///
/// [`get`] of `Value` accept any type that implements `Index`. This
/// trait is implemented for strings which are used as the index into a JSON
/// map, and for `usize` which is used as the index into a JSON array.
///
/// [`get`]: ../enum.Value.html#method.get
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_json_borrow`.
/// # Examples
///
/// ```
/// # use serde_json_borrow::Value;
/// #
/// let json_obj = r#"
/// {
///     "x": {
///         "y": ["z", "zz"]
///     }
/// }
/// "#;
///
/// let data: Value = serde_json::from_str(json_obj).unwrap();
///
/// assert_eq!(data.get("x").get("y").get(0), &Value::Str(std::borrow::Cow::Borrowed("z")));
/// assert_eq!(data.get("x").get("y").get(1), &Value::Str(std::borrow::Cow::Borrowed("zz")));
/// assert_eq!(data.get("x").get("y").get(2), &Value::Null);
///
/// assert_eq!(data.get("a"), &Value::Null);
/// assert_eq!(data.get("a").get("b"), &Value::Null);
/// ```
pub trait Index {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'a, 'ctx>(self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>>;
}

impl Index for usize {
    #[inline]
    fn index_into<'a, 'ctx>(self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>> {
        match v {
            Value::Array(vec) => vec.get(self),
            _ => None,
        }
    }
}

impl Index for &str {
    #[inline]
    fn index_into<'a, 'ctx>(self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>> {
        match v {
            Value::Object(map) => map.iter().find(|(k, _v)| k == &self).map(|(_k, v)| v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_lifetime() {
        fn get_str<'a>(v: &'a Value<'_>, k: &str) -> Option<&'a str> {
            v.get(k).as_str()
        }
        let key = String::from("key");
        let value = Value::Object(
            vec![(key.as_str().into(), Value::Str("value".into()))].into()
        );
        assert_eq!(get_str(&value, &key), Some("value"));
    }
}
