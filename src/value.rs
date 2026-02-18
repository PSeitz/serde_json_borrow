use core::fmt;
use core::hash::Hash;
use std::borrow::Cow;
use std::fmt::{Debug, Display};

use crate::index::Index;
use crate::num::{Number, N};
pub use crate::object_vec::ObjectAsVec;

/// Represents any valid JSON value.
///
/// # Example
/// ```
/// use std::io;
/// use serde_json_borrow::Value;
/// fn main() -> io::Result<()> {
///     let data = r#"{"bool": true, "key": "123"}"#;
///     let value: Value = serde_json::from_str(&data)?;
///     assert_eq!(value.get("bool"), &Value::Bool(true));
///     assert_eq!(value.get("key"), &Value::Str("123".into()));
///     Ok(())
/// }
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub enum Value<'ctx> {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Null;
    /// ```
    #[default]
    Null,

    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Bool(true);
    /// ```
    Bool(bool),

    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Number(12.5.into());
    /// ```
    Number(Number),

    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Str("ref".into());
    /// ```
    Str(Cow<'ctx, str>),

    /// Represents a JSON array.
    Array(Vec<Value<'ctx>>),

    /// Represents a JSON object.
    ///
    /// By default the map is backed by a Vec. Allows very fast deserialization.
    /// Ideal when wanting to iterate over the values, in contrast to look up by key.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// # use serde_json_borrow::ObjectAsVec;
    /// #
    /// let v = Value::Object([("key".into(), Value::Str("value".into()))].into_iter().collect::<Vec<_>>().into());
    /// ```
    Object(ObjectAsVec<'ctx>),
}

impl<'ctx> Value<'ctx> {
    /// Index into a `serde_json_borrow::Value` using the syntax `value.get(0)` or
    /// `value.get("k")`.
    ///
    /// Returns `Value::Null` if the type of `self` does not match the type of
    /// the index, for example if the index is a string and `self` is an array
    /// or a number. Also returns `Value::Null` if the given key does not exist
    /// in the map or the given index is not within the bounds of the array.
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
    /// assert_eq!(data.get("x").get("y").get(0), &Value::Str("z".into()));
    /// assert_eq!(data.get("x").get("y").get(1), &Value::Str("zz".into()));
    /// assert_eq!(data.get("x").get("y").get(2), &Value::Null);
    ///
    /// assert_eq!(data.get("a"), &Value::Null);
    /// assert_eq!(data.get("a").get("b"), &Value::Null);
    /// ```
    #[inline]
    pub fn get<I: Index>(&self, index: I) -> &Value<'ctx> {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }

    /// Returns true if `Value` is Value::Null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns true if `Value` is Value::Array.
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Returns true if `Value` is Value::Object.
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Returns true if `Value` is Value::Bool.
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns true if `Value` is Value::Number.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Returns true if `Value` is Value::Str.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::Str(_))
    }

    /// Returns true if the Value is an integer between i64::MIN and i64::MAX.
    /// For any Value on which is_i64 returns true, as_i64 is guaranteed to return the integer
    /// value.
    pub fn is_i64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the Value is an integer between zero and u64::MAX.
    /// For any Value on which is_u64 returns true, as_u64 is guaranteed to return the integer
    /// value.
    pub fn is_u64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u64(),
            _ => false,
        }
    }

    /// Returns true if the Value is a f64 number.
    pub fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    /// If the Value is an Array, returns an iterator over the elements in the array.
    pub fn iter_array(&self) -> Option<impl Iterator<Item = &Value<'_>>> {
        match self {
            Value::Array(arr) => Some(arr.iter()),
            _ => None,
        }
    }

    /// If the Value is an Object, returns an iterator over the elements in the object.
    pub fn iter_object(&self) -> Option<impl Iterator<Item = (&str, &Value<'_>)>> {
        match self {
            Value::Object(arr) => Some(arr.iter()),
            _ => None,
        }
    }

    /// If the Value is an Array, returns the associated Array. Returns None otherwise.
    pub fn as_array(&self) -> Option<&[Value<'ctx>]> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// If the Value is an Object, returns the associated Object. Returns None otherwise.
    pub fn as_object(&self) -> Option<&ObjectAsVec<'ctx>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// If the Value is a Boolean, returns the associated bool. Returns None otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// If the Value is a String, returns the associated str. Returns None otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(text) => Some(text),
            _ => None,
        }
    }

    /// If the Value is an integer, represent it as i64 if possible. Returns None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the Value is an integer, represent it as u64 if possible. Returns None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the Value is a number, represent it as f64 if possible. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.as_f64(),
            _ => None,
        }
    }
}

impl From<bool> for Value<'_> {
    fn from(val: bool) -> Self {
        Value::Bool(val)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(val: &'a str) -> Self {
        Value::Str(Cow::Borrowed(val))
    }
}

impl From<String> for Value<'_> {
    fn from(val: String) -> Self {
        Value::Str(Cow::Owned(val))
    }
}

impl<'a, T: Into<Value<'a>>> From<Vec<T>> for Value<'a> {
    fn from(val: Vec<T>) -> Self {
        Value::Array(val.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value<'a>>> From<&[T]> for Value<'a> {
    fn from(val: &[T]) -> Self {
        Value::Array(val.iter().map(Clone::clone).map(Into::into).collect())
    }
}

impl Debug for Value<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => match number.n {
                N::PosInt(n) => write!(formatter, "Number({:?})", n),
                N::NegInt(n) => write!(formatter, "Number({:?})", n),
                N::Float(n) => write!(formatter, "Number({:?})", n),
            },
            Value::Str(string) => write!(formatter, "Str({:?})", string),
            Value::Array(vec) => {
                formatter.write_str("Array ")?;
                Debug::fmt(vec, formatter)
            }
            Value::Object(map) => {
                formatter.write_str("Object ")?;
                Debug::fmt(map, formatter)
            }
        }
    }
}

// We just convert to serde_json::Value to Display
impl Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::Value::from(self.clone()))
    }
}

impl From<u64> for Value<'_> {
    fn from(val: u64) -> Self {
        Value::Number(val.into())
    }
}

impl From<i64> for Value<'_> {
    fn from(val: i64) -> Self {
        Value::Number(val.into())
    }
}

impl From<f64> for Value<'_> {
    fn from(val: f64) -> Self {
        Value::Number(val.into())
    }
}

impl From<Value<'_>> for serde_json::Value {
    fn from(val: Value) -> Self {
        match val {
            Value::Null => serde_json::Value::Null,
            Value::Bool(val) => serde_json::Value::Bool(val),
            Value::Number(val) => serde_json::Value::Number(val.into()),
            Value::Str(val) => serde_json::Value::String(val.to_string()),
            Value::Array(vals) => {
                serde_json::Value::Array(vals.into_iter().map(|val| val.into()).collect())
            }
            Value::Object(vals) => serde_json::Value::Object(vals.into()),
        }
    }
}

impl From<&Value<'_>> for serde_json::Value {
    fn from(val: &Value) -> Self {
        match val {
            Value::Null => serde_json::Value::Null,
            Value::Bool(val) => serde_json::Value::Bool(*val),
            Value::Number(val) => serde_json::Value::Number((*val).into()),
            Value::Str(val) => serde_json::Value::String(val.to_string()),
            Value::Array(vals) => {
                serde_json::Value::Array(vals.iter().map(|val| val.into()).collect())
            }
            Value::Object(vals) => serde_json::Value::Object(vals.into()),
        }
    }
}

impl<'ctx> From<&'ctx serde_json::Value> for Value<'ctx> {
    fn from(value: &'ctx serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    Value::Number(n.into())
                } else if let Some(n) = n.as_u64() {
                    Value::Number(n.into())
                } else if let Some(n) = n.as_f64() {
                    Value::Number(n.into())
                } else {
                    unreachable!()
                }
            }
            serde_json::Value::String(val) => Value::Str(Cow::Borrowed(val)),
            serde_json::Value::Array(arr) => {
                let out: Vec<Value<'ctx>> = arr.iter().map(|v| v.into()).collect();
                Value::Array(out)
            }
            serde_json::Value::Object(obj) => {
                let mut ans = ObjectAsVec::default();
                for (k, v) in obj {
                    ans.insert(k.as_str(), v.into());
                }
                Value::Object(ans)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn from_serde() {
        let value = &serde_json::json!({
            "a": 1,
            "b": "2",
            "c": [3, 4],
            "d": {"e": "alo"}
        });

        let value: Value = value.into();
        assert_eq!(value.get("a"), &Value::Number(1i64.into()));
        assert_eq!(value.get("b"), &Value::Str("2".into()));
        assert_eq!(value.get("c").get(0), &Value::Number(3i64.into()));
        assert_eq!(value.get("c").get(1), &Value::Number(4i64.into()));
        assert_eq!(value.get("d").get("e"), &Value::Str("alo".into()));
    }

    #[test]
    fn number_test() -> io::Result<()> {
        let data = r#"{"val1": 123.5, "val2": 123, "val3": -123}"#;
        let value: Value = serde_json::from_str(data)?;
        assert!(value.get("val1").is_f64());
        assert!(!value.get("val1").is_u64());
        assert!(!value.get("val1").is_i64());

        assert!(!value.get("val2").is_f64());
        assert!(value.get("val2").is_u64());
        assert!(value.get("val2").is_i64());

        assert!(!value.get("val3").is_f64());
        assert!(!value.get("val3").is_u64());
        assert!(value.get("val3").is_i64());

        assert!(value.get("val1").as_f64().is_some());
        assert!(value.get("val2").as_f64().is_some());
        assert!(value.get("val3").as_f64().is_some());

        assert!(value.get("val1").as_u64().is_none());
        assert!(value.get("val2").as_u64().is_some());
        assert!(value.get("val3").as_u64().is_none());

        assert!(value.get("val1").as_i64().is_none());
        assert!(value.get("val2").as_i64().is_some());
        assert!(value.get("val3").as_i64().is_some());

        Ok(())
    }
}
