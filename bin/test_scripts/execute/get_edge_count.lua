local trans = transaction();
local outbound = vertex("foo");
trans:create_vertex(outbound);
local inbound = vertex("foo");
trans:create_vertex(inbound);
trans:create_edge(EdgeKey.new(outbound.id, "baz", inbound.id));

local count = trans:get_edge_count(outbound.id, "baz", "outbound");
assert(count == 1);

local count = trans:get_edge_count(outbound.id, nil, "outbound");
assert(count == 1);
