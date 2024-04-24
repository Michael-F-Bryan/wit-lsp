use std::{borrow::Borrow, ops::Deref};

use crate::Text;

/// The path to a [`SourceFile`] in the [`Workspace`].
///
/// Ideally, this should only ever be passed around as an opaque identifier and
/// shown to the user. You shouldn't make any assumptions about its contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FilePath(pub Text);

impl Deref for FilePath {
    type Target = Text;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for FilePath
where
    T: Into<Text>,
{
    fn from(value: T) -> Self {
        FilePath(value.into())
    }
}

impl<T> Borrow<T> for FilePath
where
    Text: Borrow<T>,
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&FilePath> for FilePath {
    fn from(value: &FilePath) -> Self {
        value.clone()
    }
}
