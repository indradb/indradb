# IndraDB library [![Docs](https://docs.rs/indradb-lib/badge.svg)](https://docs.rs/indradb-lib)

This is the IndraDB library. Most users can use the [server](https://github.com/indradb/indradb), which provides higher-level methods for working with IndraDB databases. Linking directly against the library would be necessary if you want to make a new datastore implementation, or if you want to work at a lower-level than the server affords.

## Pluggable datastores

IndraDB stores graph data in datastores. Datastores are pluggable: there are a few built-in ones, but you can implement a new custom datastore.

### In-memory

To use the in-memory datastore, add this to your `Cargo.toml`'s dependencies section:

```toml
indradb-lib = "1"
```

### RocksDB

To use the rocksdb datastore, add this to your `Cargo.toml`'s dependencies section:

```toml
indradb-lib = { version = "1", features = ["rocksdb-datastore"] }
```

### Custom datastores

To implement a custom datastore, you need to implement the [Datastore trait](https://github.com/indradb/indradb/blob/master/lib/src/traits.rs). See the [in-memory datastore](https://github.com/indradb/indradb/blob/master/lib/src/memory/datastore.rs) for a simpler example implementation. To help you get off the ground faster, there is a standard test suite that can execute against any datastore and check for common bugs and regressions. See the [in-memory datastore tests](https://github.com/indradb/indradb/blob/master/lib/src/memory/tests.rs) for an implementation of it.
