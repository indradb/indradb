use crate::{Database, Datastore, Error};

pub fn should_sync<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    // just make sure that it runs fine
    db.sync()
}
