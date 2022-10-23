use std::io;

use crate::Value;

/// Parses a `String` into `Value`, by taking ownership of `String` and reference slices from it in
/// contrast to copying the contents.
pub struct OwnedValue {
    /// Keep owned data, to be able to safely reference it from Value<'static>
    _data: String,
    value: Value<'static>,
}

impl OwnedValue {
    pub fn parse_from(data: String) -> io::Result<Self> {
        let value: Value = serde_json::from_str(&data)?;
        let value = unsafe { extend_lifetime(value) };

        Ok(Self { _data: data, value })
    }

    /// Returns the `Value` reference.
    pub fn get_value(&self) -> &Value<'_> {
        &self.value
    }
}

unsafe fn extend_lifetime<'b>(r: Value<'b>) -> Value<'static> {
    std::mem::transmute::<Value<'b>, Value<'static>>(r)
}
