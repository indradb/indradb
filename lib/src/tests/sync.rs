use super::TestDatabase;
use crate::Datastore;

pub fn should_sync<D: Datastore>(db: &TestDatabase<D>) {
    // just make sure that it runs fine
    db.sync().unwrap();
}
