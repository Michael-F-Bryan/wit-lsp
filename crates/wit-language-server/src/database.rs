use std::fmt::Debug;

/// An implementation of [`Db`] and [`wit_compiler::Db`].
#[derive(Default)]
#[salsa::db(Jar, wit_compiler::Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Database {
            storage: self.storage.snapshot(),
        })
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database").finish_non_exhaustive()
    }
}

#[salsa::jar(db = Db)]
pub struct Jar(crate::ops::folding_range);

pub trait Db: salsa::DbWithJar<Jar> + wit_compiler::Db {
    /// Reinterpret this [`Db`] as a [`wit_compiler::Db`].
    fn as_wit(&self) -> &dyn wit_compiler::Db;

    /// Reinterpret this [`Db`] as a [`wit_compiler::Db`].
    fn as_wit_mut(&mut self) -> &mut dyn wit_compiler::Db;
}

impl<DB> Db for DB
where
    DB: salsa::DbWithJar<Jar> + salsa::DbWithJar<wit_compiler::Jar>,
{
    fn as_wit(&self) -> &dyn wit_compiler::Db {
        self
    }

    fn as_wit_mut(&mut self) -> &mut dyn wit_compiler::Db {
        self
    }
}
