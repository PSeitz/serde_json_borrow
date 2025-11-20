use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;

use serde::de::{DeserializeSeed, MapAccess, Visitor};

use crate::{Map, Value};

/// A JSON Deserializer that reuses the same map allocation for each deserialization.
///
/// # Example
///
/// ```
/// use serde_json_borrow::ReusableMap;
///
/// // Create a deserializer that will reuse the same map allocation
/// let mut reusable_map = ReusableMap::new();
///
/// let json_strs = [
///     r#"{"name":"test","value":42}"#,
///     r#"{"name":"other","other":"value"}"#,
/// ];
///
/// for json_str in json_strs {
///     // Get a guard that provides access to the map for the lifetime of json_str
///     let mapped = reusable_map.deserialize(json_str).unwrap();
///     assert!(mapped.get("name").is_some());
///     // When the guard is dropped, the ReusableMap is cleared and released for reuse
/// }
/// ```
///
/// Note that you cannot use the ReusableMap while there is a guard active:
///
/// ```rust,compile_fail
/// # use serde_json_borrow::ReusableMap;
/// let mut reusable_map = ReusableMap::new();
///
/// let json_str = r#"{"name":"test","value":42}"#;
///
/// let mapped = reusable_map.deserialize(json_str).unwrap();
/// let mapped2 = reusable_map.deserialize(json_str).unwrap(); // <-- fails
/// ```
///
/// Nor can the guard outlive the json string (or the ReusableMap):
///
/// ```rust,compile_fail
/// # use serde_json_borrow::ReusableMap;
/// let mut reusable_map = ReusableMap::new();
///
/// let json_str = r#"{"name":"test","value":42}"#;
///
/// let mapped = {
///     let string = json_str.to_string();
///     let inner = reusable_map.deserialize(&string).unwrap();
///     inner
/// };
/// ```
pub struct ReusableMap {
    /// The reusable map that persists between deserializations
    map: Map<'static>,
}

impl ReusableMap {
    /// Creates a new empty ReusableMap
    pub fn new() -> Self {
        Self {
            map: Map::default(),
        }
    }

    /// Deserializes a JSON string and returns a guard that provides safe access to the map.
    ///
    /// Returns a [`BorrowedMap`] on success, or a deserialization error on
    /// failure. The `BorrowedMap` provides safe access to the deserialized
    /// map, and must be dropped before the deserializer can be used again.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use serde_json_borrow::{ReusableMap, Value};
    /// let mut map = ReusableMap::new();
    /// let json = r#"{"name": "Alice"}"#;
    ///
    /// let guard = map.deserialize(json).unwrap();
    /// assert_eq!(guard.get("name"), Some(&Value::Str(Cow::Borrowed("Alice"))));
    /// ```
    pub fn deserialize<'json, 'deser>(
        &'deser mut self,
        json: &'json str,
    ) -> Result<BorrowedMap<'json, 'deser>, serde_json::Error> {
        let mut deserializer = serde_json::Deserializer::from_str(json);

        // SAFETY: We're using transmute to convert the map's lifetime.
        // This is safe because:
        // 1. We're tying the resulting map's lifetime to the input JSON string ('json) and this deserializer
        // 2. The BorrowedMap has a mutable reference to this JsonDeserializer, preventing deserialization while the guard exists
        // 3. The guard's lifetime parameters ensure the map can't be accessed after the JSON string is invalid
        // 4. The Guard clears the map on drop, ensuring no dangling references
        let map =
            unsafe { mem::transmute::<&mut Map<'static>, &'json mut Map<'json>>(&mut self.map) };

        let seed = JsonMapSeed { map };
        seed.deserialize(&mut deserializer)?;

        Ok(BorrowedMap {
            // SAFETY: We're using transmute to convert the map's lifetime.
            // This has the same safety guarantees as the original transmute.
            map: unsafe {
                mem::transmute::<&mut Map<'static>, &'json mut Map<'json>>(&mut self.map)
            },
            _deserializer: PhantomData,
        })
    }
}

impl Default for ReusableMap {
    fn default() -> Self {
        Self::new()
    }
}

/// A guard that provides safe access to a deserialized JSON map.
///
/// It dereferences to [`Map`], see it for the methods available on the guard.
///
/// It can only be created by the [`ReusableMap::deserialize`] method,
/// see that method for more information.
pub struct BorrowedMap<'json, 'deser> {
    /// Reference to the map with lifetime tied to the JSON string
    map: &'json mut Map<'json>,
    /// Phantom data to tie the guard's lifetime to the ReusableMap
    _deserializer: PhantomData<&'deser ()>,
}

impl<'json> Deref for BorrowedMap<'json, '_> {
    type Target = Map<'json>;

    fn deref(&self) -> &Self::Target {
        self.map
    }
}

impl Drop for BorrowedMap<'_, '_> {
    fn drop(&mut self) {
        // We clear the map to prevent dangling references from previous calls
        self.map.clear();
    }
}

/// A struct that allows us to deserialize JSON into an existing map.
struct JsonMapSeed<'json> {
    map: &'json mut Map<'json>,
}

impl<'de, 'json> DeserializeSeed<'de> for JsonMapSeed<'json>
where
    'de: 'json,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de, 'json> Visitor<'de> for JsonMapSeed<'json>
where
    'de: 'json,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<M>(self, mut access: M) -> Result<(), M::Error>
    where
        M: MapAccess<'de>,
    {
        while let Some((key, value)) = access.next_entry::<&'de str, Value<'de>>()? {
            self.map.insert(key, value);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::num::N;

    use super::*;

    #[test]
    fn test_map_cleared_on_drop() {
        let mut deserializer = ReusableMap::new();

        // First JSON - create and drop a guard
        {
            let json_str = r#"{"name":"test"}"#.to_string();
            let guard = deserializer.deserialize(&json_str).unwrap();
            assert_eq!(guard.len(), 1);
            assert!(guard.contains_key("name"));
        }

        // Second JSON - should start with a clean map
        let json_str2 = r#"{"second":"value"}"#.to_string();
        let guard2 = deserializer.deserialize(&json_str2).unwrap();

        // Verify the map was cleared by confirming it only has the new content
        assert_eq!(guard2.len(), 1);
        assert!(guard2.contains_key("second"));
        assert!(!guard2.contains_key("name"));
    }

    #[test]
    fn test_json_guard_deserialization() {
        let mut deserializer = ReusableMap::new();

        let json_str = r#"{"name":"test","value":42,"nested":{"key":"val"}}"#.to_string();

        let guard = deserializer.deserialize(&json_str).unwrap();

        // Verify the contents were properly deserialized
        assert_eq!(guard.len(), 3);
        assert_eq!(
            guard.get("name").unwrap(),
            &Value::Str(Cow::Borrowed("test"))
        );
        assert_eq!(
            guard.get("value").unwrap(),
            &Value::Number(crate::num::Number { n: N::PosInt(42) })
        );
        assert_eq!(
            guard.get("nested").unwrap(),
            &Value::Object(Map::from(vec![("key", Value::Str(Cow::Borrowed("val")))]))
        );

        // When guard is dropped, the deserializer is released
        drop(guard);

        // Deserialize again with a new guard, reusing the same map allocation
        let guard2 = deserializer.deserialize(r#"{"another":"value"}"#).unwrap();
        assert_eq!(guard2.len(), 1);
        assert_eq!(
            guard2.get("another").unwrap(),
            &Value::Str(Cow::Borrowed("value"))
        );
    }

    #[test]
    fn test_invalid_json() {
        let invalid_json = r#"{{"name":"test", invalid}}"#.to_string();
        let mut deserializer = ReusableMap::new();

        let result = deserializer.deserialize(&invalid_json);
        assert!(
            result.is_err(),
            "Deserialization of invalid JSON should fail"
        );
    }

    #[test]
    fn test_non_object_json() {
        let array_json = r#"[1, 2, 3]"#;
        let mut deserializer = ReusableMap::new();

        let result = deserializer.deserialize(array_json);
        assert!(
            result.is_err(),
            "Deserialization of JSON array should fail when expecting object"
        );
    }
}
