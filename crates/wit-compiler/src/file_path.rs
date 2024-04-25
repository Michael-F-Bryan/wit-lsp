use crate::Text;

/// The path to a [`SourceFile`] in the [`Workspace`].
///
/// Ideally, this should only ever be passed around as an opaque identifier and
/// shown to the user. You shouldn't make any assumptions about its contents.
#[salsa::interned]
pub struct FilePath {
    #[return_ref]
    pub raw_path: Text,
}
