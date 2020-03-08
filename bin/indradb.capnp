@0xa24c698a359c7c15;

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
    t @1 :Type;
    inboundId @2 :Uuid;
}

struct Vertex {
    id @0 :Uuid;
    t @1 :Type;
}

struct VertexQuery {
    union {
        range :group {
            limit @0 :UInt32;
            t @1 :Type;
            startId @2 :Uuid;
        }
        specific :group {
            ids @3 :List(Uuid);
        }
        pipe :group {
            inner @4 :EdgeQuery;
            direction @5 :EdgeDirection;
            limit @6 :UInt32;
            t @7 :Type;
        }
    }
}

struct VertexPropertyQuery {
    inner @0 :VertexQuery;
    name @1 :Text;
}

struct EdgeQuery {
    union {
        specific :group {
            keys @0 :List(EdgeKey);
        }
        pipe :group {
            inner @1 :VertexQuery;
            direction @2 :EdgeDirection;
            t @3 :Type;
            high @4 :Timestamp;
            low @5 :Timestamp;
            limit @6 :UInt32;
        }
    }
}

struct EdgePropertyQuery {
    inner @0 :EdgeQuery;
    name @1 :Text;
}

enum EdgeDirection {
    outbound @0;
    inbound @1;
}

struct Property {
    name @0 :Text;
    value @1 :Json;
}

struct VertexProperty {
    id @0 :Uuid;
    value @1 :Json;
}

struct VertexProperties {
    vertex @0 :Vertex;
    props @1 :List(Property);
}

struct EdgeProperty {
    key @0 :EdgeKey;
    value @1 :Json;
}

struct EdgeProperties {
    edge @0 :Edge;
    props @1 :List(Property);
}

struct BulkInsertItem {
    union {
        vertex :group {
            vertex @0 :Vertex;
        }
        edge :group {
            key @1 :EdgeKey;
        }
        vertexProperty :group {
            id @2 :Uuid;
            name @3 :Text;
            value @4 :Json;
        }
        edgeProperty :group {
            key @5 :EdgeKey;
            name @6 :Text;
            value @7 :Json;
        }
    }
}

interface Service {
    ping @0 () -> (ready :Bool);
    transaction @1 () -> (transaction :Transaction);
    bulkInsert @2 (items :List(BulkInsertItem)) -> (result :Void);
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
    # * `t` - Only get the count for a specified edge type.
    # * `direction`: The direction of edges to get.
    getEdgeCount @8 (id :Uuid, t :Type, direction :EdgeDirection) -> (result :UInt64);

    # Gets vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    getVertexProperties @9 (q :VertexPropertyQuery) -> (result :List(VertexProperty));

    # Sets vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    # * `value` - The property value.
    setVertexProperties @10 (q :VertexPropertyQuery, value :Json) -> (result :Void);

    # Deletes vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    deleteVertexProperties @11 (q :VertexPropertyQuery) -> (result :Void);

    # Gets edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    getEdgeProperties @12 (q :EdgePropertyQuery) -> (result :List(EdgeProperty));

    # Sets edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    # * `value` - The property value.
    setEdgeProperties @13 (q :EdgePropertyQuery, value :Json) -> (result :Void);

    # Deletes edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    deleteEdgeProperties @14 (q :EdgePropertyQuery) -> (result :Void);

    # Gets vertexes and all properties for each vertex.
    #
    # Arguments
    # * `q` - The query to run.
    getAllVertexProperties @15 (q :VertexQuery) -> (result :List(VertexProperties));

    # Gets edges and all properties for each edge.
    #
    # Arguments
    # * `q` - The query to run.
    getAllEdgeProperties @16 (q :EdgeQuery) -> (result :List(EdgeProperties));

}
