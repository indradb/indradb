@0xc656e2e7cbc5b02e;

using Timestamp = UInt64;
using Uuid = Data;
using Type = Text;
using Json = Text;

struct Edge {
    key @0 :EdgeKey;
    createdDatetime @1 :Timestamp;
}

struct EdgeKey {
    outboundId @0 :Uuid;
    type @1 :Type;
    inboundId @2 :Uuid;
}

struct Vertex {
    id @0 :Uuid;
    type @1 :Type;
}

struct VertexQuery {
    union {
        all :group {
            startId @0 :Uuid;
            limit @1 :UInt32;
        }
        vertices :group {
            ids @2 :List(Uuid);
        }
        pipe :group {
            edgeQuery @3 :EdgeQuery;
            converter @4 :EdgeDirection;
            limit @5 :UInt32;
        }
    }
}

struct EdgeQuery {
    union {
        edges :group {
            keys @0 :List(EdgeKey);
        }
        pipe :group {
            vertexQuery @1 :VertexQuery;
            converter @2 :EdgeDirection;
            typeFilter @3 :Type;
            highFilter @4 :Timestamp;
            lowFilter @5 :Timestamp;
            limit @6 :UInt32;
        }
    }
}

enum EdgeDirection {
    outbound @0;
    inbound @1;
}

struct VertexProperty {
    id @0 :Uuid;
    value @1 :Json;
}

struct EdgeProperty {
    key @0 :EdgeKey;
    value @1 :Json;
}

interface Service {
    ping @0 () -> (ready :Bool);
    transaction @1 () -> (transaction :Transaction);
}

interface Transaction {
    # Creates a new vertex. Returns whether the vertex was successfully
    # created - if this is false, it's because a vertex with the same UUID
    # already exists.
    #
    # Arguments
    # * `vertex`: The vertex to create.
    createVertex @0 (vertex :Vertex) -> (result :Bool);

    # Creates a new vertex with just a type specification. As opposed to
    # `createVertex`, this is used when you do not want to manually specify
    # the vertex's UUID. Returns the new vertex's UUID.
    #
    # Arguments
    # * `t`: The type of the vertex to create.
    createVertexFromType @1 (t :Type) -> (result :Uuid);

    # Gets a range of vertices specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    getVertices @2 (q :VertexQuery) -> (result :List(Vertex));

    # Deletes existing vertices specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    deleteVertices @3 (q :VertexQuery) -> (result :Void);

    # Gets the number of vertices in the datastore..
    getVertexCount @4 () -> (result :UInt64);

    # Creates a new edge. If the edge already exists, this will update it
    # with a new update datetime. Returns whether the edge was successfully
    # created - if this is false, it's because one of the specified vertices
    # is missing.
    #
    # Arguments
    # * `key`: The edge to create.
    createEdge @5 (key :EdgeKey) -> (result :Bool);

    # Gets a range of edges specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    getEdges @6 (q :EdgeQuery) -> (result :List(Edge));

    # Deletes a set of edges specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    deleteEdges @7 (q :EdgeQuery) -> (result :Void);

    # Gets the number of edges associated with a vertex.
    #
    # Arguments
    # * `id` - The id of the vertex.
    # * `typeFilter` - Only get the count for a specified edge type.
    # * `direction`: The direction of edges to get.
    getEdgeCount @8 (id :Uuid, typeFilter :Type, direction :EdgeDirection) -> (result :UInt64);

    # Gets vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    getVertexProperties @9 (q :VertexQuery, name :Text) -> (result :List(VertexProperty));

    # Sets vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    # * `value` - The property value.
    setVertexProperties @10 (q :VertexQuery, name :Text, value :Json) -> (result :Void);

    # Deletes vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    deleteVertexProperties @11 (q :VertexQuery, name :Text) -> (result :Void);

    # Gets edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    getEdgeProperties @12 (q :EdgeQuery, name :Text) -> (result :List(EdgeProperty));

    # Sets edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    # * `value` - The property value.
    setEdgeProperties @13 (q :EdgeQuery, name :Text, value :Json) -> (result :Void);

    # Deletes edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    deleteEdgeProperties @14 (q :EdgeQuery, name :Text) -> (result :Void);
}
