# braid [![Build Status](https://travis-ci.org/braidery/braid.svg?branch=master)](https://travis-ci.org/braidery/braid)

A graph database. This software is in the alpha state. Do not use this as your single source of truth, and do not expect peak performance.

## Features at a glance

* Support for directed, weighted, and typed graphs.
* An advanced query DSL.
* Multiple ways to work with the database:
    * Via HTTP API, and the clients that build off of it.
    * Via lua-based scripting.
    * By embedding braid directly as a library.
* Multitenancy / support for multiple accounts.
* Support for metadata.
* Pluggable underlying datastores, with built-in support for [postgres](https://www.postgresql.org/) and [rocksdb](https://github.com/facebook/rocksdb).
* Written in rust!

## Getting started

* Install [rust](https://www.rust-lang.org/en-US/install.html) 1.16+ stable or nightly.
* Make sure you have liblua5.1 installed
* If you want to use the rocksdb datastore:
    * Make sure you have gcc 5+ installed
* If you want to use the postgres datastore:
    * Make sure you have postgres 9.5+ installed
* Clone the repo: `git clone git@github.com:braidery/braid.git`.
* Build a release version: `cargo build --release`.
* Create a new account: `DATABASE_URL=rocksdb://database.rdb target/release/braid-user add email_address@gmail.com`.
* Start the server: `DATABASE_URL=rocksdb://database.rdb PORT=8000 target/release/braid-server`.
* Make a sample HTTP request to `http://localhost:8000`, with the credentials supplied when you created the account.

## Concepts

### Vertices

Vertices represent things, and have types and UUIDs. Example:
```json
{"id": "e148f973-cb90-48f8-8070-192b6928a613", "type": "user"}
```

### Edges

Edges represent relationships between things, and have types, weights, and when they were last created/updated. Example:

```json
{
    "key": {
        "outbound_id": "e148f973-cb90-48f8-8070-192b6928a613",
        "type": "likes",
        "inbound_id": "a086155e-e2f9-4bd1-86a3-bd3d7169ba03"
    },
    "weight": 1.0,
    "update_datetime": "2014-11-28 12:00:09"
}
```

### Accounts

All vertices are owned by an account. Accounts can view all data and create an arbitrary number of vertices, but they can only create edges where the outbound UUID is a vertex that they own. This facilitates sharing data between accounts in the same way the web does: you can view any page, and you can link to any page, but cannot make a page link back to you. Of course, you can always have just one account that owns all graph data if this is not necessary, and can always create separate instances of braid servers to have completely isolated accounts.

### Metadata

Metadata are JSON-based key/value items. They can be global (i.e. tied to nothing), or tied to a vertex, edge, or account. When a vertex, edge or account is deleted, its respective metadata is deleted as well.

Functionality for querying, updating, and deleting metadata is tied to the same transactions as the functionality for manipulating graph data. This means you can make updates to metadata along with graph data, with the guarantees of whatever the underlying datastore affords you.

Metadata isn't directly available via the HTTP API, and isn't subject to any sort of permissions checking. It's generally used by scripts to store data that isn't directly representable by the graph. For example, you may have a script that, in the background, is determining the nearest neighbors for vertices; you can cache the results in metadata.

### Queries

Braid offers rich query semantics for working with vertices and edges, inspired by [Gremlin](https://github.com/tinkerpop/gremlin/wiki) and [Cayley](https://github.com/cayleygraph/cayley). Vertex and edge queries can be chained together via pipes, so that an edge query can build off the results of a vertex query and vice-versa. Here are some examples.

#### Vertex queries

Get all vertices ordered by UUID, starting from the beginning, and limited to up to 1000 vertices:
```json
{"all": [null, 1000]}
```

Get all vertices ordered by UUID, starting after the UUID `22c6bbfe-a89d-4e51-96f9-cbf176aed23e`, and limited to up to 1000 vertices:
```json
{"all": ["22c6bbfe-a89d-4e51-96f9-cbf176aed23e", 1000]}
```

Get a specific vertex:
```json
{"vertex": "22c6bbfe-a89d-4e51-96f9-cbf176aed23e"}
```

Get a set of specific vertices:
```json
{
    "vertices": [
        "22c6bbfe-a89d-4e51-96f9-cbf176aed23e",
        "a4caa313-b9ec-4d44-99c9-d22084cf9889",
        "f5c25812-d178-4d89-908c-2679db17f82a"
    ]
}
```

Get the outbound vertex associated with an edge, and limited to up to 1 vertex:
```json
{
    "pipe": [{
        "edge": {
            "outbound_id": "22c6bbfe-a89d-4e51-96f9-cbf176aed23e",
            "type": "foo",
            "inbound_id": "f5c25812-d178-4d89-908c-2679db17f82a"
        }
    },
    "outbound",
    1
]}
```

#### Edge queries

Get a specific edge:
```json
{
    "edge": {
        "outbound_id": "22c6bbfe-a89d-4e51-96f9-cbf176aed23e",
        "type": "foo",
        "inbound_id": "f5c25812-d178-4d89-908c-2679db17f82a"
    }
}
```

Get a set of specific edges:
```json
{
    "edges": [{
        "outbound_id": "22c6bbfe-a89d-4e51-96f9-cbf176aed23e",
        "type": "foo",
        "inbound_id": "f5c25812-d178-4d89-908c-2679db17f82a"
    }, {
        "outbound_id": "8b986253-3033-46f0-bc0e-d3115b22b857",
        "type": "foo",
        "inbound_id": "1d92355b-93c9-424f-8b42-995da6bd2cb7"
    }, {
        "outbound_id": "1b485269-5235-4071-bdb8-5ef7ed2a76ca",
        "type": "foo",
        "inbound_id": "1dfde93f-8a2f-41c0-949d-97a2ca671d77"
    }]
}
```

Get the edges with a specified inbound UUID, and type `foo`, and limited to up to 1000 edges:
```json
{
    "pipe": [
        {"vertex": "22c6bbfe-a89d-4e51-96f9-cbf176aed23e"},
        "inbound",
        "foo",
        null,
        null,
        1000
    ]
}
```

Get the edges with a specified inbound UUID, and that were last updated before or at `2014-11-28 12:00:09`, and limited to up to 1000 edges:
```json
{
    "pipe": [
        {"vertex": "22c6bbfe-a89d-4e51-96f9-cbf176aed23e"},
        "inbound",
        null,
        "2014-11-28 12:00:09",
        null,
        1000
    ]
}
```

Get the edges with a specified inbound UUID, and that were last updated after or at `2014-11-28 12:00:09`, and limited to up to 1000 edges:
```json
{
    "pipe": [
        {"vertex": "22c6bbfe-a89d-4e51-96f9-cbf176aed23e"},
        "inbound",
        null,
        null,
        "2014-11-28 12:00:09",
        1000
    ]
}
```
