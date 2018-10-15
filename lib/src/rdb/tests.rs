#[test]
fn should_repair() {
    use super::RocksdbDatastore;
    use util::generate_random_secret;
    use std::path::Path;

    // TODO: do not hardcode the temp directory to support non-POSIX
    let path = Path::new("/tmp/test-rdb").join(generate_random_secret(8));

    // // Make sure we just initialize the database
    RocksdbDatastore::new(path.to_str().unwrap(), Some(1)).unwrap();

    // Now try to repair
    RocksdbDatastore::repair(path.to_str().unwrap(), Some(1)).unwrap();
}
