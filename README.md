<p align="center">
 	<img src="https://indradb.github.io/logo.png">
</p>

# [IndraDB](https://indradb.github.io) ![CI](https://github.com/indradb/indradb/workflows/Test/badge.svg)

A graph database written in rust.

IndraDB consists of a server and an underlying library. Most users would use the server, which is available via releases as pre-compiled binaries. But if you're a rust developer that wants to embed a graph database directly in your application, you can use the [library](https://github.com/indradb/indradb/tree/master/lib).

IndraDB's original design is heavily inspired by [TAO](https://www.cs.cmu.edu/~pavlo/courses/fall2013/static/papers/11730-atc13-bronson.pdf), facebook's graph datastore. In particular, IndraDB emphasizes simplicity of implementation and query semantics, and is similarly designed with the assumption that it may be representing a graph large enough that full graph processing is not possible. IndraDB departs from TAO (and most graph databases) in its support for properties.

For more details, see the [homepage](https://indradb.github.io).

## Features

* Support for directed and typed graphs.
* Support for queries with multiple hops.
* Cross-language support via gRPC, or direct embedding as a library.
* Support for JSON-based properties tied to vertices and edges.
* Pluggable underlying datastores, with several built-in datastores. [Postgresql is available separately](https://github.com/indradb/postgres).
* Written in rust! High performance, no GC pauses, and a higher degree of safety.

## Getting started

* [Download the latest release for your platform.](https://github.com/indradb/indradb/releases)
* Add the binaries to your `PATH`.
* Start the server: `indradb-server`

This should start the default datastore.

### Memory

By default, IndraDB starts a datastore that stores all values in-memory. This datastore is fastest for most access patterns, but has a few caveats:

* No support for graphs larger than what can be stored in-memory.
* No transactional guarantees.
* Data is only persisted to disk when explicitly requested.

If you want to use the standard datastore, don't pass a subcommand; e.g.:

```bash
indradb-server [options]
```

### RocksDB

If you want to use the rocksdb-backed datastore, use the `rocksdb` subcommand; e.g.:

```bash
indradb-server rocksdb [/path/to/rocksdb.rdb] [options]
```

### Sled

If you want to a datastore based on [sled](http://sled.rs/), use the `sled` subcommand; e.g.:

```bash
indradb-server sled [path/to/sled] [options]
```

 If the sled directory does not exist, it will be created.

**NOTE:** The sled datastore is not production-ready yet. sled itself is pre-1.0, and makes no guarantees about on-disk format stability. Upgrading IndraDB may require you to [manually migrate the sled datastore.](https://docs.rs/sled/0.34.6/sled/struct.Db.html#method.export) Additionally, there is a [standing issue](https://github.com/indradb/indradb/issues/98) that prevents the sled datastore from having the same level of safety as the RocksDB datastore.

## Install from source

If you don't want to use the pre-built releases, you can build/install from source:

* Install [rust](https://www.rust-lang.org/en-US/install.html). IndraDB should work with any of the rust variants (stable, nightly, beta.)
* Make sure you have gcc 5+ installed.
* Clone the repo: `git clone git@github.com:indradb/indradb.git`.
* Build/install it: `cargo install`.

## Running tests

Use `make test` to run the test suite. Note that this will run the full test suite across the entire workspace, including tests for all datastore implementations. Similarly, you can run `make bench` to run the full benchmarking suite.

You can filter which tests run via the `TEST_NAME` environment variable. e.g. `TEST_NAME=create_vertex make test` will run tests with `create_vertex` in the name across all datastore implementations.
