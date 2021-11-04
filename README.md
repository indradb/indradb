<p align="center">
 	<img src="https://indradb.github.io/logo.png">
</p>

# [IndraDB](https://indradb.github.io) ![CI](https://github.com/indradb/indradb/workflows/Test/badge.svg)

A graph database written in rust.

IndraDB consists of a server and an underlying library. Most users would use the server, which is available via releases as pre-compiled binaries. But if you're a rust developer that wants to embed a graph database directly in your application, you can use the [library](https://github.com/indradb/indradb/tree/master/lib).

IndraDB's original design is heavily inspired by [TAO](https://www.cs.cmu.edu/~pavlo/courses/fall2013/static/papers/11730-atc13-bronson.pdf), facebook's graph datastore. In particular, IndraDB emphasizes simplicity of implementation and query semantics, and is similarly designed with the assumption that it may be representing a graph large enough that full graph processing is not possible. IndraDB departs from TAO (and most graph databases) in its support for properties.

For more details, see the [homepage](https://indradb.github.io). See also a [complete demo of IndraDB for browsing the wikipedia article link graph.](https://github.com/indradb/wikipedia-example)

## Features

* Support for directed and typed graphs.
* Support for queries with multiple hops.
* Cross-language support via gRPC, or direct embedding as a library.
* Support for JSON-based properties tied to vertices and edges.
* Pluggable underlying datastores, with several built-in datastores. [Postgresql is available separately](https://github.com/indradb/postgres).
* Written in rust! High performance, no GC pauses, and a higher degree of safety.

## Installation

### Releases

We offer pre-compiled releases for linux and macOS.

* [Download the latest release for your platform.](https://github.com/indradb/indradb/releases)
* Add the binaries to your `PATH`.
* Start the server: `indradb-server`

This should start the default datastore.

### From source

To build and install from source:

* Install [rust](https://www.rust-lang.org/en-US/install.html). IndraDB should work with any of the rust variants (stable, nightly, beta.)
* Make sure you have gcc 5+ installed.
* Clone the repo: `git clone git@github.com:indradb/indradb.git`.
* Build/install it: `cargo install`.

### Docker

If you want to run IndraDB in docker, follow the below instructions.

#### Server 

Build the image for the server:

```bash
DOCKER_BUILDKIT=1 docker build --target server -t indradb-server .
```

Run the server:

```bash
docker run --network host --rm indradb-server -a 0.0.0.0:27615
```

#### Client

Build the image for the client:

```bash
DOCKER_BUILDKIT=1 docker build --target client -t indradb-client .
```

Run the client:

```bash
docker run --network host --rm indradb-client grpc://localhost:27615 ping
```

## Datastores

IndraDB offers several different datastores with trade-offs in durability, transaction capabilities, and performance.

### Memory

By default, IndraDB starts a datastore that stores all values in-memory. This is the fastest implementation, but there's no support for graphs larger than what can fit in-memory, and data is only persisted to disk when explicitly requested.

If you want to use the standard datastore _without_ support for persistence, don't pass a subcommand; e.g.:

```bash
indradb-server [options]
```

If you want to use the standard datastore but persist to disk:

```bash
indradb-server memory --persist-path=[/path/to/memory/image.bincode]
```

You'll need to explicitly call `Sync()` when you want to save the graph.

### RocksDB

If you want to use the rocksdb-backed datastore, use the `rocksdb` subcommand; e.g.:

```bash
indradb-server rocksdb [/path/to/rocksdb.rdb] [options]
```

### Postgres, Sled, etc.

It's possible to develop other datastores implementations in separate crates, since the IndraDB exposes the necessary traits to implement:

* Postgres is available through [indradb-postgres.](https://github.com/indradb/postgres)
* Sled is available through [indradb-sled.](https://github.com/indradb/sled)

## Testing

### Unit tests

Use `make test` to run the test suite. Note that this will run the full test suite across the entire workspace, including tests for all datastore implementations. You can filter which tests run via the `TEST_NAME` environment variable. e.g. `TEST_NAME=create_vertex make test` will run tests with `create_vertex` in the name across all datastore implementations. All unit tests will run in CI.

### Benchmarks

Microbenchmarks can be run via `make bench`.

### Fuzzing

A fuzzer is available, ensuring the the RocksDB and in-memory datastores operate identically. Run it via `make fuzz`.

### Checks

Lint and formatting checks can be run via `make check`. Equivalent checks will be run in CI.

