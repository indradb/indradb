local trans = transaction();
local outbound_id = trans:create_vertex_from_type("foo");
local inbound_id = trans:create_vertex_from_type("foo");
local key = EdgeKey.new(outbound_id, "baz", inbound_id);
trans:create_edge(key);
trans:delete_edges(EdgeQuery.edges({key}));

local edges = trans:get_edges(EdgeQuery.edges({key}));
assert(#edges == 0);