use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};

/// A cheaply cloneable, immutable string.
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Text(Arc<str>);

impl Text {
    pub fn new(s: impl Into<Arc<str>>) -> Self {
        Text(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text::new(value)
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text::new(value)
    }
}

impl From<&String> for Text {
    fn from(value: &String) -> Self {
        Text::new(value.as_str())
    }
}

impl From<&Text> for Text {
    fn from(value: &Text) -> Self {
        value.clone()
    }
}

impl From<Text> for Arc<str> {
    fn from(value: Text) -> Self {
        value.0
    }
}

impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.as_str().into()
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Text
where
    Arc<str>: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.0.as_ref()
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for Text {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&String> for Text {
    fn eq(&self, other: &&String) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<Text> for str {
    fn eq(&self, other: &Text) -> bool {
        other == self
    }
}

impl PartialEq<Text> for &str {
    fn eq(&self, other: &Text) -> bool {
        other == self
    }
}

impl PartialEq<Text> for String {
    fn eq(&self, other: &Text) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<Text> for &String {
    fn eq(&self, other: &Text) -> bool {
        self.as_str() == other
    }
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
