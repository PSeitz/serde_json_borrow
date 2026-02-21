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
/// # use std::borrow::Cow;
/// # use serde_json_borrow::Value;
/// let json_obj = r#"
/// {
///     "x": {
///         "y": ["z", "zz"]
///     }
/// }
/// "#;
///
/// let data: Value = serde_json::from_str(json_obj).unwrap();
/// let y = data.get("x").unwrap().get("y").unwrap();
/// assert_eq!(y.get(0), Some(&Value::Str(Cow::Borrowed("z"))));
/// assert_eq!(y.get(1), Some(&Value::Str(Cow::Borrowed("zz"))));
/// assert_eq!(y.get(2), None);
///
/// assert_eq!(data.get("a"), None);
/// ```
pub trait Index: private::Sealed {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'a, 'ctx: 'a>(&self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>>;
}

impl Index for usize {
    #[inline]
    fn index_into<'a, 'ctx: 'a>(&self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
}

impl Index for str {
    #[inline]
    fn index_into<'a, 'ctx: 'a>(&self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>> {
        match v {
            Value::Object(map) => map.iter().find(|(k, _v)| *k == self).map(|(_k, v)| v),
            _ => None,
        }
    }
}

impl Index for String {
    #[inline]
    fn index_into<'a, 'ctx: 'a>(&self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>> {
        self.as_str().index_into(v)
    }
}

#[cfg(feature = "cowkeys")]
impl Index for std::borrow::Cow<'_, str> {
    #[inline]
    fn index_into<'a, 'ctx: 'a>(&self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>> {
        (**self).index_into(v)
    }
}

impl<T> Index for &T where T: Index + ?Sized {
    fn index_into<'a, 'ctx: 'a>(&self, v: &'a Value<'ctx>) -> Option<&'a Value<'ctx>> {
        (**self).index_into(v)
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    #[cfg(feature = "cowkeys")]
    impl Sealed for std::borrow::Cow<'_, str> {}
    impl<T> Sealed for &T where T: ?Sized + Sealed {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "cowkeys")]
    #[test]
    fn index_cow() {
        use std::borrow::Cow;

        let key = Cow::Borrowed("key");
        let value = Value::Object(
            vec![(&*key, Value::Str("value".into()))].into()
        );
        assert_eq!(value.get(&key).and_then(Value::as_str), Some("value"));
        assert_eq!(value.get(Cow::Owned("key".into())).and_then(Value::as_str), Some("value"));
    }

    #[test]
    fn index_lifetime() {
        fn get_str<'a>(v: &'a Value<'_>, k: &str) -> Option<&'a str> {
            v.get(k).and_then(Value::as_str)
        }
        let key = String::from("key");
        let value = Value::Object(
            vec![(key.as_str(), Value::Str("value".into()))].into()
        );
        assert_eq!(get_str(&value, &key), Some("value"));
    }

    #[test]
    fn index_string() {
        let key = String::from("key");
        let value = Value::Object(
            vec![(key.as_str(), Value::Str("value".into()))].into()
        );
        assert_eq!(value.get(&key).and_then(Value::as_str), Some("value"));
        assert_eq!(value.get(key.clone()).and_then(Value::as_str), Some("value"));
    }
}
