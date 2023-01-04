use crate::{Datastore, TransactionBuilder};

pub fn should_sync<T: TransactionBuilder>(datastore: &Datastore<T>) {
    // just make sure that it runs fine
    datastore.sync().unwrap();
}
