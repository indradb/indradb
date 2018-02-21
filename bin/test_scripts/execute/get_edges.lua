local trans = transaction();
local outbound_id = trans:create_vertex("foo");
local inbound_id = trans:create_vertex("bar");
trans:create_edge(EdgeKey.new(outbound_id, "baz", inbound_id));

local edges = trans:get_edges(VertexQuery.vertices({outbound_id}):outbound_edges("baz", nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);

local edges = trans:get_edges(VertexQuery.vertices({outbound_id}):outbound_edges(nil, nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);
