#![allow(clippy::useless_conversion)]
#![allow(clippy::useless_asref)]

use std::borrow::Cow;

use crate::Value;

#[cfg(feature = "cowkeys")]
/// The string type used. Can be toggled between &str and Cow<str> via `cowstr` feature flag
pub type KeyStrType<'a> = Cow<'a, str>;

#[cfg(not(feature = "cowkeys"))]
/// The string type used. Can be toggled between &str and Cow<str> via `cowstr` feature flag
/// Cow strings
pub type KeyStrType<'a> = &'a str;

/// Represents a JSON key/value type.
///
/// For performance reasons we use a Vec instead of a Hashmap.
/// This comes with a tradeoff of slower key accesses as we need to iterate and compare.
///
/// The ObjectAsVec struct is a wrapper around a Vec of (&str, Value) pairs.
/// It provides methods to make it easy to migrate from serde_json::Value::Object or
/// serde_json::Map.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ObjectAsVec<'ctx>(pub(crate) Vec<(KeyStrType<'ctx>, Value<'ctx>)>);

impl<'ctx> From<Vec<(&'ctx str, Value<'ctx>)>> for ObjectAsVec<'ctx> {
    fn from(vec: Vec<(&'ctx str, Value<'ctx>)>) -> Self {
        Self(vec.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}

impl<'ctx> ObjectAsVec<'ctx> {
    /// Access to the underlying Vec.
    ///
    /// # Note
    /// Since KeyStrType can be changed via a feature flag avoid using `as_vec` and use other
    /// methods instead. This could be a problem with feature unification, when one crate uses it
    /// as &str and another uses it as Cow<str>, both will get Cow<str?
    #[inline]
    pub fn as_vec(&self) -> &Vec<(KeyStrType, Value)> {
        &self.0
    }

    /// Access to the underlying Vec. Keys are normalized to Cow.
    #[inline]
    pub fn into_vec(self) -> Vec<(Cow<'ctx, str>, Value<'ctx>)> {
        self.0.into_iter().map(|el| (el.0.into(), el.1)).collect()
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// ## Performance
    /// As this is backed by a Vec, this searches linearly through the Vec as may be much more
    /// expensive than a `Hashmap` for larger Objects.
    #[inline]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0
            .iter()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    /// Returns a mutable reference to the value corresponding to the key, if it exists.
    ///
    /// ## Performance
    /// As this is backed by a Vec, this searches linearly through the Vec as may be much more
    /// expensive than a `Hashmap` for larger Objects.
    #[inline]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value<'ctx>> {
        self.0
            .iter_mut()
            .find_map(|(k, v)| if *k == key { Some(v) } else { None })
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// ## Performance
    /// As this is backed by a Vec, this searches linearly through the Vec as may be much more
    /// expensive than a `Hashmap` for larger Objects.
    #[inline]
    pub fn get_key_value(&self, key: &str) -> Option<(&str, &Value)> {
        self.0.iter().find_map(|(k, v)| {
            if *k == key {
                Some((k.as_ref(), v))
            } else {
                None
            }
        })
    }

    /// An iterator visiting all key-value pairs
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(k, v)| (k.as_ref(), v))
    }

    /// Returns the number of elements in the object
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the object contains no elements
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// An iterator visiting all keys
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|(k, _)| k.as_ref())
    }

    /// An iterator visiting all values
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.0.iter().map(|(_, v)| v)
    }

    /// Returns true if the object contains a value for the specified key.
    ///
    /// ## Performance
    /// As this is backed by a Vec, this searches linearly through the Vec as may be much more
    /// expensive than a `Hashmap` for larger Objects.
    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(k, _)| *k == key)
    }

    /// Inserts a key-value pair into the object.
    /// If the object did not have this key present, `None` is returned.
    /// If the object did have this key present, the value is updated, and the old value is
    /// returned.
    ///
    /// ## Performance
    /// This operation is linear in the size of the Vec because it potentially requires iterating
    /// through all elements to find a matching key.
    #[inline]
    pub fn insert(&mut self, key: &'ctx str, value: Value<'ctx>) -> Option<Value<'ctx>> {
        for (k, v) in &mut self.0 {
            if *k == key {
                return Some(std::mem::replace(v, value));
            }
        }
        // If the key is not found, push the new key-value pair to the end of the Vec
        self.0.push((key.into(), value));
        None
    }

    /// Inserts a key-value pair into the object if the key does not yet exist, otherwise returns a
    /// mutable reference to the existing value.
    ///
    /// ## Performance
    /// This operation might be linear in the size of the Vec because it requires iterating through
    /// all elements to find a matching key, and might add to the end if not found.
    #[inline]
    pub fn insert_or_get_mut(&mut self, key: &'ctx str, value: Value<'ctx>) -> &mut Value<'ctx> {
        // get position to circumvent lifetime issue
        if let Some(pos) = self.0.iter_mut().position(|(k, _)| *k == key) {
            &mut self.0[pos].1
        } else {
            self.0.push((key.into(), value));
            &mut self.0.last_mut().unwrap().1
        }
    }

    /// Inserts a key-value pair into the object and returns the mutable reference of the inserted
    /// value.
    ///
    /// ## Note
    /// The key must not exist in the object. If the key already exists, the object will contain
    /// multiple keys afterwards.
    ///
    /// ## Performance
    /// This operation is amortized constant time, worst case linear time in the size of the Vec
    /// because it potentially requires a reallocation to grow the Vec.
    #[inline]
    pub fn insert_unchecked_and_get_mut(
        &mut self,
        key: &'ctx str,
        value: Value<'ctx>,
    ) -> &mut Value<'ctx> {
        self.0.push((key.into(), value));
        let idx = self.0.len() - 1;
        &mut self.0[idx].1
    }
}

impl<'ctx> From<ObjectAsVec<'ctx>> for serde_json::Map<String, serde_json::Value> {
    fn from(val: ObjectAsVec<'ctx>) -> Self {
        val.iter()
            .map(|(key, val)| (key.to_string(), val.into()))
            .collect()
    }
}
impl<'ctx> From<&ObjectAsVec<'ctx>> for serde_json::Map<String, serde_json::Value> {
    fn from(val: &ObjectAsVec<'ctx>) -> Self {
        val.iter()
            .map(|(key, val)| (key.to_owned(), val.into()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn test_empty_initialization() {
        let obj: ObjectAsVec = ObjectAsVec(Vec::new());
        assert!(obj.is_empty());
        assert_eq!(obj.len(), 0);
    }

    #[test]
    fn test_non_empty_initialization() {
        let obj = ObjectAsVec(vec![("key".into(), Value::Null)]);
        assert!(!obj.is_empty());
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_get_existing_key() {
        let obj = ObjectAsVec(vec![("key".into(), Value::Bool(true))]);
        assert_eq!(obj.get("key"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_get_non_existing_key() {
        let obj = ObjectAsVec(vec![("key".into(), Value::Bool(true))]);
        assert_eq!(obj.get("not_a_key"), None);
    }

    #[test]
    fn test_get_key_value() {
        let obj = ObjectAsVec(vec![("key".into(), Value::Bool(true))]);
        assert_eq!(obj.get_key_value("key"), Some(("key", &Value::Bool(true))));
    }

    #[test]
    fn test_keys_iterator() {
        let obj = ObjectAsVec(vec![
            ("key1".into(), Value::Null),
            ("key2".into(), Value::Bool(false)),
        ]);
        let keys: Vec<_> = obj.keys().collect();
        assert_eq!(keys, vec!["key1", "key2"]);
    }

    #[test]
    fn test_values_iterator() {
        let obj = ObjectAsVec(vec![
            ("key1".into(), Value::Null),
            ("key2".into(), Value::Bool(true)),
        ]);
        let values: Vec<_> = obj.values().collect();
        assert_eq!(values, vec![&Value::Null, &Value::Bool(true)]);
    }

    #[test]
    fn test_iter() {
        let obj = ObjectAsVec(vec![
            ("key1".into(), Value::Null),
            ("key2".into(), Value::Bool(true)),
        ]);
        let pairs: Vec<_> = obj.iter().collect();
        assert_eq!(
            pairs,
            vec![("key1", &Value::Null), ("key2", &Value::Bool(true))]
        );
    }

    #[test]
    fn test_into_vec() {
        let obj = ObjectAsVec(vec![("key".into(), Value::Null)]);
        let vec = obj.into_vec();
        assert_eq!(vec, vec![("key".into(), Value::Null)]);
    }

    #[test]
    fn test_contains_key() {
        let obj = ObjectAsVec(vec![("key".into(), Value::Bool(false))]);
        assert!(obj.contains_key("key"));
        assert!(!obj.contains_key("no_key"));
    }

    #[test]
    fn test_insert_new() {
        let mut obj = ObjectAsVec::default();
        assert_eq!(
            obj.insert("key1", Value::Str(Cow::Borrowed("value1"))),
            None
        );
        assert_eq!(obj.len(), 1);
        assert_eq!(obj.get("key1"), Some(&Value::Str(Cow::Borrowed("value1"))));
    }

    #[test]
    fn test_insert_update() {
        let mut obj = ObjectAsVec(vec![(
            "key1".into(),
            Value::Str(Cow::Borrowed("old_value1")),
        )]);
        assert_eq!(
            obj.insert("key1", Value::Str(Cow::Borrowed("new_value1"))),
            Some(Value::Str(Cow::Borrowed("old_value1")))
        );
        assert_eq!(obj.len(), 1);
        assert_eq!(
            obj.get("key1"),
            Some(&Value::Str(Cow::Borrowed("new_value1")))
        );
    }

    #[test]
    fn test_insert_multiple_types() {
        let mut obj = ObjectAsVec::default();
        obj.insert("boolean", Value::Bool(true));
        obj.insert("number", Value::Number(3.14.into()));
        obj.insert("string", Value::Str(Cow::Borrowed("Hello")));
        obj.insert("null", Value::Null);

        assert_eq!(
            obj.insert(
                "array",
                Value::Array(vec![Value::Number(1_u64.into()), Value::Null])
            ),
            None
        );
        assert_eq!(obj.len(), 5);
        assert_eq!(obj.get("boolean"), Some(&Value::Bool(true)));
        assert_eq!(obj.get("number"), Some(&Value::Number(3.14.into())));
        assert_eq!(obj.get("string"), Some(&Value::Str(Cow::Borrowed("Hello"))));
        assert_eq!(obj.get("null"), Some(&Value::Null));
        assert_eq!(
            obj.get("array"),
            Some(&Value::Array(vec![
                Value::Number(1_u64.into()),
                Value::Null
            ]))
        );
    }
}
