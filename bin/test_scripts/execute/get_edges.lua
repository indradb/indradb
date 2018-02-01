local queries = require("../shared/queries");

local outbound_id = trans:create_vertex("foo");
local inbound_id = trans:create_vertex("bar");
trans:create_edge(queries.EdgeKey.new(outbound_id, "baz", inbound_id), 0.5);

local edges = trans:get_edges(queries.VertexQuery.vertices({outbound_id}):outbound_edges("baz", nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);
assert(edges[1].weight == 0.5);

local edges = trans:get_edges(queries.VertexQuery.vertices({outbound_id}):outbound_edges(nil, nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);
assert(edges[1].weight == 0.5);
