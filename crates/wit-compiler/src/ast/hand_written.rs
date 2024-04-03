use crate::ast::{Builtins, HasSource, Identifier, Semver, UserDefinedType};

macro_rules! literal_types {
    ($($ty:ident),* $(,)?) => {
        $(
            impl $ty<'_> {
                pub fn value(self, src: &str) -> &str {
                    self.utf8_text(src)
                }
            }
        )*
    };
}

literal_types!(Identifier, Builtins, UserDefinedType, Semver);
