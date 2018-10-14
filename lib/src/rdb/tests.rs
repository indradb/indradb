use super::RocksdbDatastore;
use super::instance::get_options;

#[test]
fn should_repair() {
    let (path, max_open_files) = get_options();

    // // Make sure we just initialize the database
    RocksdbDatastore::new(&path, Some(max_open_files));

    // Now try to repair
    RocksdbDatastore::repair(&path, Some(max_open_files)).unwrap();
}
