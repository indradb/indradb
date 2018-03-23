local trans = transaction();
local outbound = vertex("foo");
trans:create_vertex(outbound);
local inbound = vertex("foo");
trans:create_vertex(inbound);
trans:create_edge(EdgeKey.new(outbound.id, "baz", inbound.id));

local edges = trans:get_edges(VertexQuery.vertices({outbound.id}):outbound_edges("baz", nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound.id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound.id);

local edges = trans:get_edges(VertexQuery.vertices({outbound.id}):outbound_edges(nil, nil, nil, 10));
assert(#edges == 1);
assert(edges[1].key.outbound_id == outbound.id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound.id);
