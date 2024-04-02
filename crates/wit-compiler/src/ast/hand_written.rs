use crate::ast::{AstNode, Builtins, Identifier, Semver, UserDefinedType};

macro_rules! literal_types {
    ($($ty:ident),* $(,)?) => {
        $(
            impl $ty<'_> {
                pub fn value(self, src: &str) -> &str {
                    self.syntax()
                        .utf8_text(src.as_bytes())
                        .expect("source wasn't valid UTF-8")
                }
            }
        )*
    };
}

literal_types!(Identifier, Builtins, UserDefinedType, Semver);
