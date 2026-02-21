use std::io;
use std::ops::Deref;

use crate::Value;

/// Parses a `String` into `Value`, by taking ownership of `String` and reference slices from it.
///
/// With [`crate::Value`], your lifetime is tied to the lifetime of the
/// passed `str`. This means that the `Value` can only be used as long as the original `str` is
/// valid. With [`OwnedValue`], you get a owned Value instead.
///
/// Note: `OwnedValue` does not implement `Deserialize`, as it is not intended to be used for
/// deserialization. It is designed to be used when you already have a `String` containing JSON
/// data, and you want to parse it into a `Value` without worrying about lifetimes.
///
/// ## Example
/// ```
/// use serde_json_borrow::OwnedValue;
/// use serde_json_borrow::Value;
/// let raw_json = r#"{"name": "John", "age": 30}"#;
/// let owned_value = OwnedValue::from_string(raw_json.to_string()).unwrap();
/// assert_eq!(owned_value.get("name"), Some(&Value::Str("John".into())));
/// assert_eq!(owned_value.get("age"), Some(&Value::Number(30_u64.into())));
/// ```
///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedValue {
    /// Keep owned data, to be able to safely reference it from Value<'static>
    _data: String,
    value: Value<'static>,
}

impl OwnedValue {
    /// Validates `&[u8]` for utf-8 and parses it into a [crate::Value].
    pub fn from_slice(data: &[u8]) -> io::Result<Self> {
        let data = String::from_utf8(data.to_vec())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
        Self::from_string(data)
    }

    /// Takes serialized JSON `&str` and parses it into a [crate::Value].
    ///
    /// Clones the passed str.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(json_str: &str) -> io::Result<Self> {
        let json_str = json_str.to_string();
        Self::from_string(json_str)
    }

    /// Takes serialized JSON `String` and parses it into a [crate::Value].
    pub fn from_string(json_str: String) -> io::Result<Self> {
        let value: Value = serde_json::from_str(&json_str)?;
        let value = unsafe { extend_lifetime(value) };
        Ok(Self {
            _data: json_str,
            value,
        })
    }

    /// Takes serialized JSON `String` and parses it into a [crate::Value].
    pub fn parse_from(json_str: String) -> io::Result<Self> {
        Self::from_string(json_str)
    }

    /// Returns the `Value` reference.
    pub fn get_value(&self) -> &Value<'_> {
        &self.value
    }
}

impl Deref for OwnedValue {
    type Target = Value<'static>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

unsafe fn extend_lifetime<'b>(r: Value<'b>) -> Value<'static> {
    unsafe { std::mem::transmute::<Value<'b>, Value<'static>>(r) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test reading from the internal Value via Deref.
    #[test]
    fn test_deref_access() {
        let raw_json = r#"{"name": "John", "age": 30}"#;
        let owned_value = OwnedValue::from_string(raw_json.to_string()).unwrap();

        assert_eq!(owned_value.get("name"), Some(&Value::Str("John".into())));
        assert_eq!(owned_value.get("age"), Some(&Value::Number(30_u64.into())));
    }

    /// Test that clone clones OwnedValue
    #[test]
    fn test_deref_clone() {
        let raw_json = r#"{"name": "John", "age": 30}"#;
        let owned_value = OwnedValue::from_string(raw_json.to_string()).unwrap();
        let owned_value = owned_value.clone();

        assert_eq!(owned_value.get("name"), Some(&Value::Str("John".into())));
        assert_eq!(owned_value.get("age"), Some(&Value::Number(30_u64.into())));
    }
}
