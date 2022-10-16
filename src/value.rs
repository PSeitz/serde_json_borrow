use core::{
    fmt,
    hash::{Hash, Hasher},
};
use std::fmt::Debug;

use fnv::FnvHashMap;

use crate::index::Index;

/// Represents any valid JSON value.
///
/// See the [`serde_json::value` module documentation](self) for usage examples.
#[derive(Clone, Eq, PartialEq)]
pub enum Value<'ctx> {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Null;
    /// ```
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
    /// let v = Value::Str("ref");
    /// ```
    Str(&'ctx str),

    /// Represents a JSON array.
    ///
    Array(Vec<Value<'ctx>>),

    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json_borrow::Value;
    /// #
    /// let v = Value::Object([("key", Value::Str("value"))].into_iter().collect());
    /// ```
    Object(FnvHashMap<&'ctx str, Value<'ctx>>),
}

impl<'ctx> Value<'ctx> {
    /// Index into a `serde_json::Value` using the syntax `value[0]` or
    /// `value["k"]`.
    ///
    /// Returns `Value::Null` if the type of `self` does not match the type of
    /// the index, for example if the index is a string and `self` is an array
    /// or a number. Also returns `Value::Null` if the given key does not exist
    /// in the map or the given index is not within the bounds of the array.
    ///
    /// For retrieving deeply nested values, you should have a look at the
    /// `Value::pointer` method.
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
    /// assert_eq!(data.get("x").get("y").get(0), &Value::Str("z"));
    /// assert_eq!(data.get("x").get("y").get(1), &Value::Str("zz"));
    /// assert_eq!(data.get("x").get("y").get(2), &Value::Null);
    ///
    /// assert_eq!(data.get("a"), &Value::Null);
    /// assert_eq!(data.get("a").get("b"), &Value::Null);
    /// ```
    pub fn get<I: Index<'ctx>>(&'ctx self, index: I) -> &'ctx Value<'ctx> {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<'ctx> std::fmt::Debug for Value<'ctx> {
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

/// Represents a JSON number, whether integer or floating point.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Number {
    n: N,
}

#[derive(Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => a == b,
            _ => false,
        }
    }
}

// Implementing Eq is fine since any float values are always finite.
impl Eq for N {}

impl Hash for N {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            N::PosInt(i) => i.hash(h),
            N::NegInt(i) => i.hash(h),
            N::Float(f) => {
                if f == 0.0f64 {
                    // There are 2 zero representations, +0 and -0, which
                    // compare equal but have different bits. We use the +0 hash
                    // for both so that hash(+0) == hash(-0).
                    0.0f64.to_bits().hash(h);
                } else {
                    f.to_bits().hash(h);
                }
            }
        }
    }
}

impl From<u64> for Number {
    fn from(val: u64) -> Self {
        Self { n: N::PosInt(val) }
    }
}

impl From<i64> for Number {
    fn from(val: i64) -> Self {
        Self { n: N::NegInt(val) }
    }
}

impl From<f64> for Number {
    fn from(val: f64) -> Self {
        Self { n: N::Float(val) }
    }
}
