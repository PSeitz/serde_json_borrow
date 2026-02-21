#![allow(clippy::useless_conversion)]
#![allow(clippy::useless_asref)]

use std::borrow::Cow;

use crate::Value;

#[cfg(feature = "cowkeys")]
/// The string type used. Can be toggled between `&str` and `Cow<str>` via `cowstr` feature flag
pub type KeyStrType<'a> = crate::cowstr::CowStr<'a>;

#[cfg(not(feature = "cowkeys"))]
/// The string type used. Can be toggled between `&str` and `Cow<str>` via `cowstr` feature flag
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

impl<'ctx, K, V> From<Vec<(K, V)>> for ObjectAsVec<'ctx>
where
    K: Into<KeyStrType<'ctx>>,
    V: Into<Value<'ctx>>,
{
    fn from(vec: Vec<(K, V)>) -> Self {
        Self::from_iter(vec)
    }
}

impl<'ctx, K, V, const N: usize> From<[(K, V); N]> for ObjectAsVec<'ctx>
where
    K: Into<KeyStrType<'ctx>>,
    V: Into<Value<'ctx>>
{
    #[inline]
    fn from(value: [(K, V); N]) -> Self {
        Self::from_iter(value)
    }
}

impl<'ctx, K, V> FromIterator<(K, V)> for ObjectAsVec<'ctx>
where
    K: Into<KeyStrType<'ctx>>,
    V: Into<Value<'ctx>>
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(iter.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
    }
}

impl<'ctx> ObjectAsVec<'ctx> {
    /// Access to the underlying Vec.
    ///
    /// # Note
    /// Since KeyStrType can be changed via a feature flag avoid using `as_vec` and use other
    /// methods instead. This could be a problem with feature unification, when one crate uses it
    /// as `&str` and another uses it as `Cow<str>`, both will get `Cow<str>`
    #[inline]
    pub fn as_vec(&self) -> &Vec<(KeyStrType<'ctx>, Value<'ctx>)> {
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
    pub fn get(&self, key: &str) -> Option<&Value<'ctx>> {
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
    pub fn get_key_value(&self, key: &str) -> Option<(&str, &Value<'ctx>)> {
        self.0.iter().find_map(|(k, v)| {
            if *k == key {
                Some((k.as_ref(), v))
            } else {
                None
            }
        })
    }

    /// Finds an [`ObjectEntry`] in the Map by key.
    ///
    /// This method allows you to obtain both the value and its position in the underlying Vec.
    ///
    /// Similar to [`ObjectAsVec::get_key_value`], but returns an [`ObjectEntry`] instead of
    /// a tuple.
    ///
    /// ## Performance
    ///
    /// As this is backed by a Vec, this searches linearly through the Vec as may be much more
    /// expensive than a `Hashmap` for larger Objects.
    ///
    /// The returned `index` can be used with [`ObjectAsVec::get_key_value_at`] for future
    /// O(1) access.
    ///
    /// ## Example
    ///
    /// ```
    /// # use serde_json_borrow::{ObjectAsVec, Value};
    /// # let obj = ObjectAsVec::from(vec![("name", Value::Str("John".into()))]);
    /// let entry = obj.get_entry("name").unwrap();
    /// println!("Found '{}={}' at index {}", entry.key, entry.value, entry.index);
    /// ```
    #[inline]
    pub fn get_entry(&self, key: &str) -> Option<ObjectEntry<'_, 'ctx>> {
        self.0.iter().enumerate().find_map(|(index, (k, v))| {
            if *k == key {
                Some(ObjectEntry {
                    index,
                    key: k.as_ref(),
                    value: v,
                })
            } else {
                None
            }
        })
    }

    /// Retrieves an entry directly by its index in the underlying Vec.
    ///
    /// This method provides O(1) access to entries when the index is known,
    /// avoiding the linear search required by [`ObjectAsVec::get_key_value`] if you have
    /// already looked up the entry using [`ObjectAsVec::get_entry`] or otherwise have
    /// found its index.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use serde_json_borrow::{ObjectAsVec, Value};
    /// # let obj = ObjectAsVec::from(vec![("name", Value::Str("John".into()))]);
    /// let entry = obj.get_entry("name").unwrap();
    /// if let Some((key, value)) = obj.get_key_value_at(entry.index) {
    ///     println!("Found entry: {} = {:?}", key, value);
    /// }
    /// ```
    #[inline]
    pub fn get_key_value_at(&self, index: usize) -> Option<(&str, &Value<'ctx>)> {
        self.0.get(index).map(|(k, v)| (k.as_ref(), v))
    }

    /// An iterator visiting all key-value pairs
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value<'ctx>)> {
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
    pub fn values(&self) -> impl Iterator<Item = &Value<'ctx>> {
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
    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Value<'ctx>>
    where
        K: Into<KeyStrType<'ctx>>,
        V: Into<Value<'ctx>>,
    {
        let key = key.into();
        let value = value.into();
        for (k, v) in &mut self.0 {
            if *k == key {
                return Some(std::mem::replace(v, value));
            }
        }
        // If the key is not found, push the new key-value pair to the end of the Vec
        self.0.push((key, value));
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

/// An entry in a JSON object with its position in the underlying Vec, key, and value.
///
/// This struct is returned by the [`ObjectAsVec::get_entry`] method.
///
/// The index can be used with [`ObjectAsVec::get_key_value_at`] for direct access to entries
/// without searching by key.
pub struct ObjectEntry<'a, 'ctx> {
    /// The position in the underlying Vec
    pub index: usize,
    /// The key string
    pub key: &'a str,
    /// The Value reference
    pub value: &'a Value<'ctx>,
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
    use crate::num::Number;

    #[test]
    fn test_empty_initialization() {
        let obj: ObjectAsVec = ObjectAsVec(Vec::new());
        assert!(obj.is_empty());
        assert_eq!(obj.len(), 0);
    }

    #[test]
    fn test_initialization_from_array() {
        let obj = ObjectAsVec::from([
            ("a", 0u64),
            ("b", 1u64),
            ("c", 2u64),
        ]);

        assert_eq!(obj.len(), 3);
        assert_eq!(obj.get("a"), Some(&Value::Number(0u64.into())));
        assert_eq!(obj.get("b"), Some(&Value::Number(1u64.into())));
        assert_eq!(obj.get("c"), Some(&Value::Number(2u64.into())));
    }

    #[test]
    fn test_initialization_from_vec() {
        let obj = ObjectAsVec::from(vec![
            ("a", 0u64),
            ("b", 1u64),
            ("c", 2u64),
        ]);

        assert_eq!(obj.len(), 3);
        assert_eq!(obj.get("a"), Some(&Value::Number(0u64.into())));
        assert_eq!(obj.get("b"), Some(&Value::Number(1u64.into())));
        assert_eq!(obj.get("c"), Some(&Value::Number(2u64.into())));
    }

    #[test]
    fn test_initialization_from_iter() {
        let names = "abcde";
        let iter = (0usize..)
            .take(5)
            .map(|i| (&names[i..i + 1], Value::Number(Number::from(i as u64))));

        let obj = ObjectAsVec::from_iter(iter);

        assert_eq!(obj.len(), 5);
        assert_eq!(obj.get("a"), Some(&Value::Number(0u64.into())));
        assert_eq!(obj.get("b"), Some(&Value::Number(1u64.into())));
        assert_eq!(obj.get("c"), Some(&Value::Number(2u64.into())));
        assert_eq!(obj.get("d"), Some(&Value::Number(3u64.into())));
        assert_eq!(obj.get("e"), Some(&Value::Number(4u64.into())));
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
        obj.insert("number", Value::Number(1.23.into()));
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
        assert_eq!(obj.get("number"), Some(&Value::Number(1.23.into())));
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

    #[test]
    fn test_get_entry() {
        let obj = ObjectAsVec::from(vec![
            ("key1", Value::Number(42u64.into())),
            ("key2", Value::Bool(true)),
            ("key3", Value::Str(Cow::Borrowed("value"))),
        ]);

        let entry = obj.get_entry("key2").unwrap();
        assert_eq!(entry.index, 1);
        assert_eq!(entry.key, "key2");
        assert_eq!(entry.value, &Value::Bool(true));

        // non-existing key
        assert!(obj.get_entry("nonexistent").is_none());
    }

    #[test]
    fn test_get_key_value_at() {
        let obj = ObjectAsVec::from(vec![
            ("key1", Value::Number(42u64.into())),
            ("key2", Value::Bool(true)),
            ("key3", Value::Str(Cow::Borrowed("value"))),
        ]);

        // Test valid index
        let (key, value) = obj.get_key_value_at(2).unwrap();
        assert_eq!(key, "key3");
        assert_eq!(value, &Value::Str(Cow::Borrowed("value")));

        // invalid index should be none
        assert!(obj.get_key_value_at(3).is_none());
    }

    #[test]
    fn test_entry_index_usage_with_enumerate_find() {
        let obj = ObjectAsVec::from(vec![
            ("name", Value::Str(Cow::Borrowed("John"))),
            ("age", Value::Number(30u64.into())),
            ("city", Value::Str(Cow::Borrowed("New York"))),
        ]);

        let idx = obj
            .iter()
            .enumerate()
            .find(|(_, (key, _))| *key == "city")
            .map(|(idx, _)| idx)
            .unwrap();

        // ensure that the found object matches the searched for object
        let (key, _) = obj.get_key_value_at(idx).unwrap();
        assert_eq!(key, "city");
    }
}
