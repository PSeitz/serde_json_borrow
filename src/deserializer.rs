use serde::de::{self, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;

use crate::num::N;
use crate::{KeyStrType, Value};

impl<'de> IntoDeserializer<'de, de::value::Error> for &'de Value<'_> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> Deserializer<'de> for &'de Value<'_> {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(*b),
            Value::Number(n) => match n.n {
                N::PosInt(u) => visitor.visit_u64(u),
                N::NegInt(i) => visitor.visit_i64(i),
                N::Float(f) => visitor.visit_f64(f),
            },
            Value::Str(s) => visitor.visit_borrowed_str(s),
            Value::Array(arr) => {
                let seq = SeqDeserializer::new(arr);
                visitor.visit_seq(seq)
            }
            Value::Object(map) => {
                let map = MapDeserializer::new(map.as_vec().as_slice());
                visitor.visit_map(map)
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        match self {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("deserialize_enum is not yet supported"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

// Helper struct to deserialize sequences (arrays).
struct SeqDeserializer<'a, 'ctx> {
    iter: std::slice::Iter<'a, Value<'ctx>>,
}

impl<'a, 'ctx> SeqDeserializer<'a, 'ctx> {
    fn new(slice: &'a [Value<'ctx>]) -> Self {
        SeqDeserializer { iter: slice.iter() }
    }
}

impl<'de, 'a: 'de, 'ctx: 'de> SeqAccess<'de> for SeqDeserializer<'a, 'ctx> {
    type Error = de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where T: de::DeserializeSeed<'de> {
        self.iter
            .next()
            .map(|value| seed.deserialize(value))
            .transpose()
    }
}

// Helper struct to deserialize maps (objects).
struct MapDeserializer<'a, 'ctx> {
    iter: std::slice::Iter<'a, (KeyStrType<'ctx>, Value<'ctx>)>,
    value: Option<&'a Value<'ctx>>,
}

impl<'a, 'ctx> MapDeserializer<'a, 'ctx> {
    fn new(map: &'a [(KeyStrType<'ctx>, Value<'ctx>)]) -> Self {
        MapDeserializer {
            iter: map.iter(),
            value: None,
        }
    }
}

impl<'de, 'a: 'de, 'ctx: 'de> MapAccess<'de> for MapDeserializer<'a, 'ctx> {
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where K: de::DeserializeSeed<'de> {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            seed.deserialize(de::value::BorrowedStrDeserializer::new(key))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where V: de::DeserializeSeed<'de> {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::de::value::Error as DeError;
    use serde::de::{IgnoredAny, IntoDeserializer};
    use serde::Deserialize;

    use crate::num::N;
    use crate::Value;

    // Basic deserialization test for null value
    #[test]
    fn test_deserialize_null() {
        let value = Value::Null;
        let deserialized: Option<i32> = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, None);
    }

    // Test deserialization of boolean value
    #[test]
    fn test_deserialize_bool() {
        let value = Value::Bool(true);
        let deserialized: bool = Deserialize::deserialize(&value).unwrap();
        assert!(deserialized);
    }

    #[test]
    fn test_into_deserializer_bool() {
        let value = Value::Bool(true);

        // Convert Value to deserializer using IntoDeserializer
        let deserializer = (&value).into_deserializer();
        let deserialized: bool = Deserialize::deserialize(deserializer).unwrap();

        assert!(deserialized);
    }

    // Test deserialization of integer (i64) value
    #[test]
    fn test_deserialize_i64() {
        let value = Value::Number(N::NegInt(-42).into());
        let deserialized: i64 = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, -42);
    }

    // Test deserialization of unsigned integer (u64) value
    #[test]
    fn test_deserialize_u64() {
        let value = Value::Number(N::PosInt(42).into());
        let deserialized: u64 = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, 42);
    }

    // Test deserialization of floating point (f64) value
    #[test]
    fn test_deserialize_f64() {
        let value = Value::Number(N::Float(42.5).into());
        let deserialized: f64 = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, 42.5);
    }

    // Test deserialization of string value
    #[test]
    fn test_deserialize_str() {
        let value = Value::Str("Hello".into());
        let deserialized: String = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, "Hello");
    }

    // Test deserialization of optional value when null
    #[test]
    fn test_deserialize_option_none() {
        let value = Value::Null;
        let deserialized: Option<i64> = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, None);
    }

    // Test deserialization of optional value when present
    #[test]
    fn test_deserialize_option_some() {
        let value = Value::Number(N::PosInt(42).into());
        let deserialized: Option<u64> = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, Some(42));
    }

    // Test deserialization of an array (sequence of values)
    #[test]
    fn test_deserialize_array() {
        let value = Value::Array(vec![
            Value::Number(N::PosInt(1).into()),
            Value::Number(N::PosInt(2).into()),
            Value::Number(N::PosInt(3).into()),
        ]);

        let deserialized: Vec<u64> = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, vec![1, 2, 3]);
    }

    // Test deserialization of a map (object)
    #[test]
    fn test_deserialize_map() {
        let value = Value::Object(
            vec![
                ("key1", Value::Number(N::PosInt(1).into())),
                ("key2", Value::Number(N::PosInt(2).into())),
                ("key3", Value::Number(N::PosInt(3).into())),
            ]
            .into(),
        );

        let deserialized: std::collections::HashMap<String, u64> =
            Deserialize::deserialize(&value).unwrap();

        let mut expected = std::collections::HashMap::new();
        expected.insert("key1".to_string(), 1);
        expected.insert("key2".to_string(), 2);
        expected.insert("key3".to_string(), 3);

        assert_eq!(deserialized, expected);
    }

    // Test deserialization of a tuple
    #[test]
    fn test_deserialize_tuple() {
        let value = Value::Array(vec![
            Value::Number(N::PosInt(1).into()),
            Value::Str("Hello".into()),
        ]);

        let deserialized: (u64, String) = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, (1, "Hello".to_string()));
    }

    // Test deserialization of a nested structure (array within an object)
    #[test]
    fn test_deserialize_nested() {
        let value = Value::Object(
            vec![(
                "numbers",
                Value::Array(vec![
                    Value::Number(N::PosInt(1).into()),
                    Value::Number(N::PosInt(2).into()),
                    Value::Number(N::PosInt(3).into()),
                ]),
            )]
            .into(),
        );

        #[derive(Deserialize, Debug, PartialEq)]
        struct Nested {
            numbers: Vec<u64>,
        }

        let deserialized: Nested = Deserialize::deserialize(&value).unwrap();
        assert_eq!(
            deserialized,
            Nested {
                numbers: vec![1, 2, 3]
            }
        );
    }

    // Test deserialization of a newtype struct
    #[derive(Debug, Deserialize, PartialEq)]
    struct NewtypeStruct(u64);

    #[test]
    fn test_deserialize_newtype_struct() {
        let value = Value::Number(N::PosInt(42).into());
        let deserialized: NewtypeStruct = Deserialize::deserialize(&value).unwrap();
        assert_eq!(deserialized, NewtypeStruct(42));
    }

    #[test]
    fn test_deserialize_ignored_any_with_string() {
        let value = Value::Str("Ignored".into());

        let _deserialized: IgnoredAny = Deserialize::deserialize(&value).unwrap();
    }

    // Test deserialization failure (for an unsupported type like enum)
    #[test]
    fn test_deserialize_enum_fails() {
        let value = Value::Str("EnumVariant".into());
        let result: Result<(), DeError> = Deserialize::deserialize(&value);
        assert!(result.is_err());
    }
}
