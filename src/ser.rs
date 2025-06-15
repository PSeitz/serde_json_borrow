use serde::ser::{Serialize, Serializer};

use crate::num::{Number, N};
use crate::ownedvalue::OwnedValue;
use crate::value::Value;
use crate::Map;

impl Serialize for Value<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::Str(s) => serializer.serialize_str(s),
            Value::Array(v) => serializer.collect_seq(v),
            Value::Object(m) => m.serialize(serializer),
        }
    }
}
impl Serialize for Map<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

impl Serialize for OwnedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Value::serialize(self.get_value(), serializer)
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PosInt(n) => serializer.serialize_u64(n),
            N::NegInt(n) => serializer.serialize_i64(n),
            N::Float(n) => serializer.serialize_f64(n),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn serialize_json_test() {
        let json_obj =
            r#"{"bool":true,"string_key":"string_val","float":1.23,"i64":-123,"u64":123}"#;

        let val1: crate::Value = serde_json::from_str(json_obj).unwrap();
        let deser1: String = serde_json::to_string(&val1).unwrap();
        assert_eq!(deser1, json_obj);
    }
}
