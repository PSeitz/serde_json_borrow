use std::borrow::Cow;

use crate::{ObjectAsVec, Value};

#[test]
fn as_vec_mut_moves_native_vector_and_borrowed_storage() {
    let input = String::from(r#"{"key":"value"}"#);
    let Value::Object(mut object) = serde_json::from_str(&input).unwrap() else {
        panic!("expected object");
    };
    let entries_ptr = object.as_vec().as_ptr().cast::<()>();
    let capacity = object.as_vec().capacity();

    let entries = std::mem::take(object.as_vec_mut());
    assert!(object.is_empty());
    drop(object);
    assert_eq!(entries.as_ptr().cast::<()>(), entries_ptr);
    assert_eq!(entries.capacity(), capacity);

    let (key, value) = entries.into_iter().next().unwrap();
    let key: Cow<'_, str> = key.into();
    let Cow::Borrowed(key) = key else {
        panic!("key should remain borrowed");
    };
    let Value::Str(Cow::Borrowed(value)) = value else {
        panic!("value should remain borrowed");
    };
    assert_eq!(key, "key");
    assert_eq!(key.as_ptr(), input[2..5].as_ptr());
    assert_eq!(value, "value");
    assert_eq!(value.as_ptr(), input[8..13].as_ptr());
}

#[test]
fn as_vec_mut_reorders_and_appends_without_normalizing_duplicates() {
    let Value::Object(mut object) =
        serde_json::from_str(r#"{"z":false,"b":null,"z":true}"#).unwrap()
    else {
        panic!("expected object");
    };
    assert_eq!(object.keys().collect::<Vec<_>>(), ["z", "b", "z"]);
    let entries_ptr = object.as_vec().as_ptr().cast::<()>();
    let entries = object.as_vec_mut();
    assert_eq!(entries.as_ptr().cast::<()>(), entries_ptr);
    entries.swap(0, 2);
    entries.push(("a".into(), Value::Null));
    entries.push(("z".into(), Value::Null));

    assert_eq!(object.keys().collect::<Vec<_>>(), ["z", "b", "z", "a", "z"]);
    assert_eq!(object.get("z"), Some(&Value::Bool(true)));
    assert_eq!(
        serde_json::to_string(&object).unwrap(),
        r#"{"z":true,"b":null,"z":false,"a":null,"z":null}"#
    );
}

#[test]
fn as_vec_mut_moves_escaped_owned_value_without_cloning() {
    let Value::Object(mut object) = serde_json::from_str(r#"{"key":"line\nbreak"}"#).unwrap()
    else {
        panic!("expected object");
    };
    let Some(Value::Str(Cow::Owned(value))) = object.get("key") else {
        panic!("escaped value should be owned");
    };
    let value_ptr = value.as_ptr();
    let entry = object.as_vec_mut().pop().unwrap();
    let mut destination = ObjectAsVec::default();
    destination.as_vec_mut().push(entry);
    drop(object);

    let Some(Value::Str(Cow::Owned(value))) = destination.get("key") else {
        panic!("escaped value should remain owned");
    };
    assert_eq!(value, "line\nbreak");
    assert_eq!(value.as_ptr(), value_ptr);
}

#[cfg(feature = "cowkeys")]
#[test]
fn as_vec_mut_moves_escaped_owned_key_without_cloning() {
    let input = String::from(r#"{"line\nbreak":"value"}"#);
    let Value::Object(mut object) = serde_json::from_str(&input).unwrap() else {
        panic!("expected object");
    };
    let (key, _) = &object.as_vec()[0];
    let Cow::Owned(key) = &key.0 else {
        panic!("escaped key should be owned");
    };
    let key_ptr = key.as_ptr();
    let entries = std::mem::take(object.as_vec_mut());
    drop(object);

    let mut destination = ObjectAsVec::default();
    *destination.as_vec_mut() = entries;
    let (key, value) = destination.as_vec_mut().pop().unwrap();
    let Cow::Owned(key) = key.0 else {
        panic!("escaped key should remain owned");
    };
    assert_eq!(key, "line\nbreak");
    assert_eq!(key.as_ptr(), key_ptr);
    let Value::Str(Cow::Borrowed(value)) = value else {
        panic!("unescaped value should remain borrowed");
    };
    assert_eq!(value, "value");
    assert_eq!(value.as_ptr(), input[16..21].as_ptr());
}
