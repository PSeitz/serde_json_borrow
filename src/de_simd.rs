use std::borrow::Cow;

use halfbrown::HashMap;
use simd_json::{ObjectHasher, StaticNode, ValueBuilder};

use crate::{value::Number, ObjectAsVec, Value};

impl<'a> ValueBuilder<'a> for Value<'a> {
    #[inline]
    fn object_with_capacity(capacity: usize) -> Self {
        Value::Object(ObjectAsVec(Vec::with_capacity(capacity * 2)))
    }

    #[inline]
    fn array_with_capacity(capacity: usize) -> Self {
        Value::Array(Vec::with_capacity(capacity))
    }

    fn null() -> Self {
        Value::Null
    }
}
impl<'ctx> From<Vec<Value<'ctx>>> for Value<'ctx> {
    #[inline]
    fn from(values: Vec<Value<'ctx>>) -> Self {
        Value::Array(values)
    }
}

impl<'ctx> From<HashMap<&'ctx str, Value<'ctx>, ObjectHasher>> for Value<'ctx> {
    #[inline]
    fn from(hash_map: HashMap<&'ctx str, Value<'ctx>, ObjectHasher>) -> Self {
        let converted_vec = hash_map
            .into_iter()
            .map(|(k, v)| (k.into(), v))
            .collect::<Vec<_>>();

        Value::Object(ObjectAsVec(converted_vec))
    }
}

impl From<StaticNode> for Value<'_> {
    #[inline]
    fn from(value: StaticNode) -> Self {
        match value {
            StaticNode::I64(value) => Value::Number(Number::from(value)),
            StaticNode::U64(value) => Value::Number(Number::from(value)),
            StaticNode::F64(value) => Value::Number(Number::from(value)),
            StaticNode::Bool(value) => Value::Bool(value),
            StaticNode::Null => Value::Null,
        }
    }
}

impl From<i8> for Value<'_> {
    #[inline]
    fn from(value: i8) -> Self {
        Value::Number(Number::from(value as i64))
    }
}

impl From<i16> for Value<'_> {
    #[inline]
    fn from(value: i16) -> Self {
        Value::Number(Number::from(value as i64))
    }
}

impl From<i32> for Value<'_> {
    #[inline]
    fn from(value: i32) -> Self {
        Value::Number(Number::from(value as i64))
    }
}

impl From<i64> for Value<'_> {
    #[inline]
    fn from(value: i64) -> Self {
        Value::Number(Number::from(value))
    }
}

impl From<u8> for Value<'_> {
    #[inline]
    fn from(value: u8) -> Self {
        Value::Number(Number::from(value as u64))
    }
}

impl From<u16> for Value<'_> {
    #[inline]
    fn from(value: u16) -> Self {
        Value::Number(Number::from(value as u64))
    }
}

impl From<u32> for Value<'_> {
    #[inline]
    fn from(value: u32) -> Self {
        Value::Number(Number::from(value as u64))
    }
}

impl From<u64> for Value<'_> {
    #[inline]
    fn from(value: u64) -> Self {
        Value::Number(Number::from(value))
    }
}

impl From<f32> for Value<'_> {
    #[inline]
    fn from(value: f32) -> Self {
        Value::Number(Number::from(value as f64))
    }
}

impl From<f64> for Value<'_> {
    #[inline]
    fn from(value: f64) -> Self {
        Value::Number(Number::from(value))
    }
}

impl From<bool> for Value<'_> {
    #[inline]
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<()> for Value<'_> {
    #[inline]
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl From<String> for Value<'_> {
    #[inline]
    fn from(value: String) -> Self {
        Value::Str(Cow::Owned(value))
    }
}

impl<'input> From<&'input str> for Value<'input> {
    #[inline]
    fn from(value: &'input str) -> Self {
        Value::Str(Cow::Borrowed(value))
    }
}

impl<'input> From<Cow<'input, str>> for Value<'input> {
    #[inline]
    fn from(value: Cow<'input, str>) -> Self {
        Value::Str(value)
    }
}
