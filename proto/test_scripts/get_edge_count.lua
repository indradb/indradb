local trans = transaction();
local outbound_id = trans:create_vertex_from_type("foo");
local inbound_id = trans:create_vertex_from_type("foo");
trans:create_edge(EdgeKey.new(outbound_id, "baz", inbound_id));

local count = trans:get_edge_count(outbound_id, "baz", "outbound");
assert(count == 1);

local count = trans:get_edge_count(outbound_id, nil, "outbound");
assert(count == 1);