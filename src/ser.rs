use serde::ser::{Serialize, Serializer};

use crate::owned::OwnedValue;
use crate::value::{Number, Value, N};

impl<'ctx> Serialize for Value<'ctx> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::Str(s) => serializer.serialize_str(s),
            Value::Array(v) => serializer.collect_seq(v),
            Value::Object(m) => serializer.collect_map(m.iter().map(|(k, v)| (k, v))),
        }
    }
}

impl Serialize for OwnedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        Value::serialize(self.get_value(), serializer)
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self.n {
            N::PosInt(n) => serializer.serialize_u64(n),
            N::NegInt(n) => serializer.serialize_i64(n),
            N::Float(n) => serializer.serialize_f64(n),
        }
    }
}
