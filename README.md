<p align="center">
 	<img src="https://indradb.github.io/logo.png">
</p>

# [IndraDB](https://indradb.github.io)

[![Test](https://github.com/indradb/indradb/actions/workflows/test.yml/badge.svg)](https://github.com/indradb/indradb/actions/workflows/test.yml)
[![crates.io](https://img.shields.io/crates/v/indradb-lib.svg)](https://crates.io/crates/indradb-lib)
[![Released API docs](https://docs.rs/indradb-lib/badge.svg)](https://docs.rs/indradb-lib)

A graph database written in rust.

IndraDB consists of a server and an underlying library. Most users would use the server, which is available via releases as pre-compiled binaries. But if you're a rust developer that wants to embed a graph database directly in your application, you can use the [library](https://github.com/indradb/indradb/tree/master/lib).

IndraDB's original design is heavily inspired by [TAO](https://www.cs.cmu.edu/~pavlo/courses/fall2013/static/papers/11730-atc13-bronson.pdf), facebook's graph datastore. In particular, IndraDB emphasizes simplicity of implementation and query semantics, and is similarly designed with the assumption that it may be representing a graph large enough that full graph processing is not possible. IndraDB departs from TAO (and most graph databases) in its support for properties.

For more details, see the [homepage](https://indradb.github.io). See also a [complete demo of IndraDB for browsing the wikipedia article link graph.](https://github.com/indradb/wikipedia-example)

## Features

* Directed and typed graphs.
* JSON-based properties tied to vertices and edges.
* Queries with multiple hops, and queries on indexed properties.
* Cross-language support via gRPC, or direct embedding as a library.
* Pluggable underlying datastores, with several built-in datastores. [Postgresql](https://github.com/indradb/postgres) and [sled](https://github.com/indradb/sled) are available separately.
* Written in rust! High performance, no GC pauses, and a higher degree of safety.

## Usage

IndraDB offers a variety ways to work with it: as a server with cross-language support, as a rust library, and via CLI. What follows are a few examples of each use case.

### Server

The server uses [gRPC](https://grpc.io/) to facilitate cross-language support. gRPC supports many languages; see the [official list](https://grpc.io/docs/languages/), though many more are unofficially supported as well. We have official bindings available for python and rust. These examples will require you to have a running server, e.g. to start an in-memory server, simply run `indradb-server`.

#### Python

Python bindings are available [here](https://github.com/indradb/python-client) and published to pypi as `indradb`. An example:

```python
import indradb
import uuid

# Connect to the server and make sure it's up
client = indradb.Client("localhost:27615")
client.ping()

# Create a couple of vertices
out_v = indradb.Vertex(uuid.uuid4(), "person")
in_v = indradb.Vertex(uuid.uuid4(), "movie")
client.create_vertex(out_v)
client.create_vertex(in_v)

# Add an edge between the vertices
key = indradb.EdgeKey(out_v.id, "bar", in_v.id)
client.create_edge(key)

# Query for the edge
results = list(client.get_edges(indradb.SpecificEdgeQuery(key))
self.assertEqual(len(results), 1)
self.assertEqual(results[0].key, key)
```

For further reference, see the [docs](https://indradb.github.io/python-client/indradb/) and [python bindings tests](https://github.com/indradb/python-client/tree/master/tests).

#### Rust

The gRPC bindings library is available as [`indradb-proto`](https://crates.io/crates/indradb-proto). An example:

```rust
use indradb_proto as proto;

// Connect to the server and make sure it's up
let mut client = proto::Client::new("grpc://127.0.0.1:27615".try_into()?).await?;
client.ping().await?;

// Create a couple of vertices
let out_v = indradb::Vertex::new(indradb::Identifier::new("person")?);
let in_v = indradb::Vertex::new(indradb::Identifier::new("movie")?);
client.create_vertex(&out_v).await?;
client.create_vertex(&in_v).await?;

// Add an edge between the vertices
let key = indradb::EdgeKey::new(out_v.id, indradb::Identifier::new("likes")?, in_v.id);
client.create_edge(&key).await?;

// Query for the edge
let e = client.get_edges(indradb::SpecificEdgeQuery::single(key.clone()).into()).await?;
assert_eq!(e.len(), 1);
assert_eq!(key, e[0].key);
```

The rust gRPC bindings library is built to closely mirror the rust library. But if you're using 100% rust, and don't need a server, you can skip all the gRPC rigmarole and just use the rust library directly. For further reference, see the [docs](https://docs.rs/indradb-proto/latest/indradb_proto/) and the [wikipedia indexing example](https://github.com/indradb/wikipedia-example), which heavily relies on `indradb-proto`.

#### Other languages

If you're looking to contribute, adding bindings for your favorite language is a great way to start! The gRPC/protobuf definitions are [here](https://github.com/indradb/indradb/blob/master/proto/indradb.proto).

### Rust library

Add IndraDB to your `Cargo.toml`:

```toml
indradb-lib = { version = "*", features = ["rocksdb-datastore"] }
```

(You might want to pin the version, or not include the RocksDB datastore and only support in-memory.)

Here's a brief example:

```rust
use indradb::{EdgeKey, MemoryDatastore, Identifier, Vertex, SpecificEdgeQuery, SpecificVertexQuery};

// Create an in-memory datastore
let mut datastore = MemoryDatastore::default();

// Create a couple of vertices
let out_v = Vertex::new(Identifier::new("person")?);
let in_v = Vertex::new(Identifier::new("movie")?);
datastore.create_vertex(&out_v)?;
datastore.create_vertex(&in_v)?;

// Add an edge between the vertices
let key = EdgeKey::new(out_v.id, Identifier::new("likes")?, in_v.id);
datastore.create_edge(&key)?;

// Query for the edge
let e = datastore.get_edges(SpecificEdgeQuery::single(key.clone()).into())?;
assert_eq!(e.len(), 1);
assert_eq!(key, e[0].key);
```

For further reference, see the [docs](https://docs.rs/indradb-lib/latest/indradb/) and [library tests](https://github.com/indradb/indradb/tree/master/lib/src/tests).

### CLI

The CLI interacts with a running server.

First start the server: `indradb-server`.

Then, e.g. count the number of vertices: `indradb-client grpc://127.0.0.1:27615 count vertex`.

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

## Plugins

The IndraDB server includes support for plugins to extend functionality available to clients. Plugins are loaded via dynamically linked libraries.

See the [hello world plugin](https://github.com/indradb/indradb/tree/master/plugins/hello_world) and [naive vertex plugin](https://github.com/indradb/indradb/tree/master/plugins/naive_vertex_count) for demonstrations of how to author plugins.

To include plugins, see the `--plugins` argument for `indradb-server`, e.g. `indradb-server --plugins=plugins/*.so`. They are then callable via the gRPC `ExecutePlugin` function.

## Testing

### Unit tests

Use `make test` to run the test suite. Note that this will run the full test suite across the entire workspace, including tests for all datastore implementations. You can filter which tests run via the `TEST_NAME` environment variable. e.g. `TEST_NAME=create_vertex make test` will run tests with `create_vertex` in the name across all datastore implementations. All unit tests will run in CI.

### Benchmarks

Microbenchmarks can be run via `make bench`.

### Fuzzing

A fuzzer is available, ensuring the the RocksDB and in-memory datastores operate identically. Run it via `make fuzz`.

### Checks

Lint and formatting checks can be run via `make check`. Equivalent checks will be run in CI.

