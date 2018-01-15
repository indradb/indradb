# IndraDB library

This is the IndraDB library. Most users can use the [server](https://github.com/indradb/indradb), which provides higher-level methods for working with IndraDB databases. Linking directly against the library would be necessary if you want to make a new datastore implementation, or if you want to work at a lower-level than the server affords.

## Pluggable datastores

IndraDB stores graph data in datastores. Datastores are pluggable: there is built in support for in-memory-only, postgres, and rocksdb, but you can implement a new custom datastore.

### In-Memory

To use the in-memory datastore, add this to your `Cargo.toml`:

```toml
[dependencies.indradb]
git = "https://github.com/indradb/indradb"
```

### Postgres

To use the postgres datastore, add this to your `Cargo.toml`:

```toml
[dependencies.indradb]
git = "https://github.com/indradb/indradb"
features = ["postgres-datastore"]
```

### RocksDB

To use the rocksdb datastore, add this to your `Cargo.toml`:

```toml
[dependencies.indradb]
git = "https://github.com/indradb/indradb"
features = ["rocksdb-datastore"]
```

### Custom datastores

To implement a custom datastore, you need to implement the [Datastore](https://indradb.github.io/apis/lib/indradb/trait.Datastore.html) and [Transaction](https://indradb.github.io/apis/lib/indradb/trait.Transaction.html) traits. See the [postgres](https://github.com/indradb/lib/blob/develop/src/pg/datastore.rs) and [rocksdb](https://github.com/indradb/lib/blob/develop/src/rdb/datastore.rs) datastores as examples.

To help you get off the ground faster, we've defined some standard tests that can execute against any datastore and check for common bugs and regressions. See the [postgres datastore tests](https://github.com/indradb/indradb/blob/develop/src/pg/tests.rs) for an example.

## Running tests

Run `./test.sh`.

## Running benchmarks

Run `./test.sh --bench`.

