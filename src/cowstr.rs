use std::borrow::Cow;
use std::ops::Deref;

use serde::Deserialize;

/// A wrapper around `Cow<str>` that implements `Deserialize` and can deserialize
/// string keys into Cow::Borrowed when possible.
///
/// This is because serde always deserializes strings into `Cow::Owned`.
/// https://github.com/serde-rs/serde/issues/1852#issuecomment-559517427
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct CowStr<'a>(#[serde(borrow)] pub Cow<'a, str>);

impl Deref for CowStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<&str> for CowStr<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<'a> From<&'a str> for CowStr<'a> {
    fn from(s: &'a str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl From<String> for CowStr<'_> {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl<'a> From<Cow<'a, str>> for CowStr<'a> {
    fn from(s: Cow<'a, str>) -> Self {
        Self(s)
    }
}

impl<'a> From<CowStr<'a>> for Cow<'a, str> {
    fn from(s: CowStr<'a>) -> Self {
        s.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(CowStr::from("text"), "text");
        assert_eq!(CowStr::from(Cow::Borrowed("text")), "text");
        assert_eq!(CowStr::from(String::from("text")), "text");
    }
}
