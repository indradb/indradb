use crate::{Database, Datastore};

pub fn should_sync<D: Datastore>(db: &Database<D>) {
    // just make sure that it runs fine
    db.sync().unwrap();
}
