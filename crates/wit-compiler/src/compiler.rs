use std::fmt::Debug;

/// The WAI language's compiler frontend.
#[derive(Default)]
#[salsa::db(crate::Jar)]
pub struct Compiler {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Compiler {}

impl salsa::ParallelDatabase for Compiler {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Compiler {
            storage: self.storage.snapshot(),
        })
    }
}

impl Debug for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Compiler { storage: _ } = self;

        struct Hidden;
        impl Debug for Hidden {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("_")
            }
        }

        f.debug_struct("Compiler")
            .field("storage", &Hidden)
            .finish_non_exhaustive()
    }
}
