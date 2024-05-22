use crate::Value;

/// For performance reasons we use a Vec instead of a Hashmap.
///
/// This comes with a tradeoff of slower key accesses as we need to iterate and compare.
///
/// The ObjectAsVec struct is a wrapper around a Vec of (&str, Value) pairs.
/// It provides methods to make it easy to migrate from serde_json::Value::Object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectAsVec<'ctx>(pub Vec<(&'ctx str, Value<'ctx>)>);

impl<'ctx> From<Vec<(&'ctx str, Value<'ctx>)>> for ObjectAsVec<'ctx> {
    fn from(vec: Vec<(&'ctx str, Value<'ctx>)>) -> Self {
        Self(vec)
    }
}

impl<'ctx> ObjectAsVec<'ctx> {
    /// Access to the underlying Vec
    #[inline]
    pub fn as_vec(&self) -> &Vec<(&str, Value)> {
        &self.0
    }

    /// Access to the underlying Vec
    #[inline]
    pub fn into_vec(self) -> Vec<(&'ctx str, Value<'ctx>)> {
        self.0
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

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// ## Performance
    /// As this is backed by a Vec, this searches linearly through the Vec as may be much more
    /// expensive than a `Hashmap` for larger Objects.
    #[inline]
    pub fn get_key_value(&self, key: &str) -> Option<(&str, &Value)> {
        self.0
            .iter()
            .find_map(|(k, v)| if *k == key { Some((*k, v)) } else { None })
    }

    /// An iterator visiting all key-value pairs
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(k, v)| (*k, v))
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
        self.0.iter().map(|(k, _)| *k)
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
}

impl<'ctx> IntoIterator for ObjectAsVec<'ctx> {
    type Item = (&'ctx str, Value<'ctx>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_initialization() {
        let obj: ObjectAsVec = ObjectAsVec(Vec::new());
        assert!(obj.is_empty());
        assert_eq!(obj.len(), 0);
    }

    #[test]
    fn test_non_empty_initialization() {
        let obj = ObjectAsVec(vec![("key", Value::Null)]);
        assert!(!obj.is_empty());
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_get_existing_key() {
        let obj = ObjectAsVec(vec![("key", Value::Bool(true))]);
        assert_eq!(obj.get("key"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_get_non_existing_key() {
        let obj = ObjectAsVec(vec![("key", Value::Bool(true))]);
        assert_eq!(obj.get("not_a_key"), None);
    }

    #[test]
    fn test_get_key_value() {
        let obj = ObjectAsVec(vec![("key", Value::Bool(true))]);
        assert_eq!(obj.get_key_value("key"), Some(("key", &Value::Bool(true))));
    }

    #[test]
    fn test_keys_iterator() {
        let obj = ObjectAsVec(vec![("key1", Value::Null), ("key2", Value::Bool(false))]);
        let keys: Vec<_> = obj.keys().collect();
        assert_eq!(keys, vec!["key1", "key2"]);
    }

    #[test]
    fn test_values_iterator() {
        let obj = ObjectAsVec(vec![("key1", Value::Null), ("key2", Value::Bool(true))]);
        let values: Vec<_> = obj.values().collect();
        assert_eq!(values, vec![&Value::Null, &Value::Bool(true)]);
    }

    #[test]
    fn test_iter() {
        let obj = ObjectAsVec(vec![("key1", Value::Null), ("key2", Value::Bool(true))]);
        let pairs: Vec<_> = obj.iter().collect();
        assert_eq!(
            pairs,
            vec![("key1", &Value::Null), ("key2", &Value::Bool(true))]
        );
    }

    #[test]
    fn test_into_vec() {
        let obj = ObjectAsVec(vec![("key", Value::Null)]);
        let vec = obj.into_vec();
        assert_eq!(vec, vec![("key", Value::Null)]);
    }

    #[test]
    fn test_contains_key() {
        let obj = ObjectAsVec(vec![("key", Value::Bool(false))]);
        assert!(obj.contains_key("key"));
        assert!(!obj.contains_key("no_key"));
    }
}
