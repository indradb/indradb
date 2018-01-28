local queries = require("queries");

local outbound_id = create_vertex(trans, "foo");
local inbound_id = create_vertex(trans, "bar");
create_edge(trans, queries.EdgeKey.new(outbound_id, "baz", inbound_id));

local edges = get_edges(trans, queries.VertexQuery.vertices({outbound_id}):outbound_edges("baz", nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);

local edges = get_edges(trans, queries.VertexQuery.vertices({outbound_id}):outbound_edges(nil, nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);
