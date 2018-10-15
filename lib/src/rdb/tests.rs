#[test]
fn should_repair() {
    use super::RocksdbDatastore;
    use util::generate_temporary_path;
    use std::path::Path;

    let path = generate_temporary_path();

    // // Make sure we just initialize the database
    RocksdbDatastore::new(&path, Some(1)).unwrap();

    // Now try to repair
    RocksdbDatastore::repair(&path, Some(1)).unwrap();
}
