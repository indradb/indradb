use crate::Datastore;

pub fn should_sync<D: Datastore>(datastore: &D) {
    // just make sure that it runs fine
    datastore.sync().unwrap();
}
