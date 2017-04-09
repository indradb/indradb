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

## Environment variables

The application is configured via these environment variables:

* `DATABASE_URL` - The connection string to the underlying database. Examples:
    * For a postgres datastore: `postgres://user:pass@localhost:5432/database-name`.
    * For a rocksdb datastore: `rocksdb://braid.rdb`. This will store data in the directory `./braid.rdb`.
* `PORT` - The port to run the application on. Defaults to `8000`.
* `SECRET` - The postgres implementation uses this as a [pepper](https://en.wikipedia.org/wiki/Pepper_%28cryptography%29) for increased security. Defaults to an empty string.
* `BRAID_SCRIPT_ROOT` - The directory housing the lua scripts. Defaults to `./scripts`.

## Scripting

The server has a lua-based scripting layer, which allows you to run several requests without the network overhead of repeated HTTP API requests. Scripts are executed via a request to the HTTP API. See `test_scripts` for the test scripts as examples. You can reuse `test_scripts/queries.lua` to help build vertex and edge queries in lua scripts more easily.

Scripts have access to these globals:
* `account_id` - The UUID of the account that is executing the script.
* `arg` - The JSON body of the request that called the script.
* All transaction methods are exposed as global functions. All requests in the script are run through a single transaction.

## HTTP API

If you're using python, you can use the [python client](https://github.com/braidery/python-client) rather than interacting with the HTTP API directly.

### Authentication

The HTTP API uses basic authentication. The username is the user UUID, and the password is the secret. These are returned by `braid-user` when you add a new user.

### Status messages

The HTTP API uses these return status codes:

* `200` - If the request went through fine.
* `401` - If authentication failed.
* `404` - If the endpoint was not found. Note that, due to a current shortcoming in [iron router](https://github.com/iron/router), this is used in place of `405` as well.
* `500` - When a server error occurs.

### Endpoints

#### Create/update a single edge

`PUT /edge/:outbound_id/:type/:inbound_id?weight=:weight`: Creates a new edge, or updates an existing one.

URL parameters:
* `outbound_id` - The vertex UUID from which the edge is outbounding.
* `type` - The edge type.
* `inbound_id` - The vertex UUID into which the edge is inbounding.

Query parameters:
* `weight` - The edge weight, which must be between -1.0 and 1.0.

Returns nothing.

#### Get edges

`GET /edge?q=:query`: Gets the edges that match the query.

Query parameters:
* `query` - The edge query JSON.

Returns a JSON array of edges that match the query.

#### Edge count

`GET /edge?action=count&q=:query`: Gets the number of edges that match the query.

Query parameters:
* `query` - The edge query JSON.

Returns the number of edges that match the query.

#### Update edges

`PUT /edge?q=:query&weight=:weight`: Updates the edges that match the query with the supplied weight.

Query parameters:
* `query` - The edge query JSON.
* `weight` - The weight to set the edges to, which must be between -1.0 and 1.0.

Returns nothing.

#### Delete edges

`DELETE /edge?q=:query`: Deletes edges that match the query.

Query parameters:
* `query` - The edge query JSON.

Returns nothing.

#### Get vertices

`GET /vertex?q=:query`: Get vertices that match a query.

Query parameters:
* `query` - The vertex query JSON.

Returns a JSON array of the vertices.

#### Create vertex

`POST /vertex?type=:type` - Creates a new vertex with the given type.

Query parameters:
* `type` - The type of the vertex. Must be less than 256 characters long, and can only contain letters, numbers, dashes, and underscores.

Returns the vertex UUID.

#### Update vertices

`PUT /vertex?q=:query&type=:type`: Updates existing vertices that match a query.

Query parameters:
* `query` - The vertex query JSON.
* `type` - Matching vertices will be updated to have this type. Must be less than 256 characters long, and can only contain letters, numbers, dashes, and underscores.

Returns nothing.

#### Delete vertices

`DELETE /vertex?q=:query`: Deletes vertices that match a query.

Query parameters:
* `query` - The vertex query JSON.

Returns nothing.

#### Run a script

`POST /script/:filename`: Runs a script with the given filename.

URL parameters:
* `filename` - The lua script to run.

The request body can be JSON. When it is, it is passed to the script as `arg`.

Returns the value returned by the script, serialized to JSON.

#### Transactions

`POST /transaction`: Executes several requests in a single transaction.

The request body must be a JSON array of requests.

Returns a JSON array of responses.
