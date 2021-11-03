use crate::Datastore;

pub fn should_sync<D: Datastore>(datastore: &mut D) {
    // just make sure that it runs fine
    datastore.sync().unwrap();
}
