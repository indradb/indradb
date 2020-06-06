@0x9f3d0bfdd2b7ebff;

using VertexId = UInt64;
using Type = Text;
using Json = Text;

struct Edge {
    outboundId @0 :VertexId;
    t @1 :Type;
    inboundId @2 :VertexId;
}

struct Vertex {
    id @0 :VertexId;
    t @1 :Type;
}

struct VertexQuery {
    union {
        range :group {
            limit @0 :UInt32;
            t @1 :Type;
            startId @2 :VertexId;
        }
        specific :group {
            ids @3 :List(VertexId);
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
            edges @0 :List(Edge);
        }
        pipe :group {
            inner @1 :VertexQuery;
            direction @2 :EdgeDirection;
            t @3 :Type;
            limit @4 :UInt32;
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
    id @0 :VertexId;
    value @1 :Json;
}

struct VertexProperties {
    vertex @0 :Vertex;
    props @1 :List(Property);
}

struct EdgeProperty {
    edge @0 :Edge;
    value @1 :Json;
}

struct EdgeProperties {
    edge @0 :Edge;
    props @1 :List(Property);
}

struct BulkInsertItem {
    union {
        vertex :group {
            t @0 :Type;
        }
        edge :group {
            edge @1 :Edge;
        }
        vertexProperty :group {
            id @2 :VertexId;
            name @3 :Text;
            value @4 :Json;
        }
        edgeProperty :group {
            edge @5 :Edge;
            name @6 :Text;
            value @7 :Json;
        }
    }
}

struct BulkInsertResult {
    startId @0 :VertexId;
    endId @1 :VertexId;
}

interface Service {
    ping @0 () -> (ready :Bool);
    transaction @1 () -> (transaction :Transaction);
    bulkInsert @2 (items :List(BulkInsertItem)) -> (result :BulkInsertResult);
}

interface Transaction {
    # Creates a new vertex. Returns the new vertex's ID.
    #
    # Arguments
    # * `t`: The type of the vertex to create.
    createVertex @0 (t :Type) -> (result :VertexId);

    # Gets a range of vertices specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    getVertices @1 (q :VertexQuery) -> (result :List(Vertex));

    # Deletes existing vertices specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    deleteVertices @2 (q :VertexQuery) -> (result :Void);

    # Gets the number of vertices in the datastore..
    getVertexCount @3 () -> (result :UInt64);

    # Creates a new edge. Returns whether the edge was successfully
    # created - if this is false, it's because one of the specified vertices
    # is missing.
    #
    # Arguments
    # * `edge`: The edge to create.
    createEdge @4 (edge :Edge) -> (result :Bool);

    # Gets a range of edges specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    getEdges @5 (q :EdgeQuery) -> (result :List(Edge));

    # Deletes a set of edges specified by a query.
    #
    # Arguments
    # * `q` - The query to run.
    deleteEdges @6 (q :EdgeQuery) -> (result :Void);

    # Gets the number of edges associated with a vertex.
    #
    # Arguments
    # * `id` - The id of the vertex.
    # * `t` - Only get the count for a specified edge type.
    # * `direction`: The direction of edges to get.
    getEdgeCount @7 (id :VertexId, t :Type, direction :EdgeDirection) -> (result :UInt64);

    # Gets vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    getVertexProperties @8 (q :VertexPropertyQuery) -> (result :List(VertexProperty));

    # Sets vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    # * `value` - The property value.
    setVertexProperties @9 (q :VertexPropertyQuery, value :Json) -> (result :Void);

    # Deletes vertex properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    deleteVertexProperties @10 (q :VertexPropertyQuery) -> (result :Void);

    # Gets edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    getEdgeProperties @11 (q :EdgePropertyQuery) -> (result :List(EdgeProperty));

    # Sets edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    # * `value` - The property value.
    setEdgeProperties @12 (q :EdgePropertyQuery, value :Json) -> (result :Void);

    # Deletes edge properties.
    #
    # Arguments
    # * `q` - The query to run.
    # * `name` - The property name.
    deleteEdgeProperties @13 (q :EdgePropertyQuery) -> (result :Void);

    # Gets vertexes and all properties for each vertex.
    #
    # Arguments
    # * `q` - The query to run.
    getAllVertexProperties @14 (q :VertexQuery) -> (result :List(VertexProperties));

    # Gets edges and all properties for each edge.
    #
    # Arguments
    # * `q` - The query to run.
    getAllEdgeProperties @15 (q :EdgeQuery) -> (result :List(EdgeProperties));

}
