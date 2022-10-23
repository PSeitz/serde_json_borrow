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
///

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
pub trait Index<'v> {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into(self, v: &'v Value<'v>) -> Option<&Value<'v>>;
}

impl<'v> Index<'v> for usize {
    fn index_into(self, v: &'v Value<'v>) -> Option<&Value<'v>> {
        match v {
            Value::Array(vec) => vec.get(self),
            _ => None,
        }
    }
}

impl<'v, 'a: 'v> Index<'v> for &'a str {
    fn index_into(self, v: &'v Value<'v>) -> Option<&Value<'v>> {
        match v {
            Value::Object(map) => map.iter().find(|(k, _v)| k == &self).map(|(_k, v)| v),
            _ => None,
        }
    }
}
