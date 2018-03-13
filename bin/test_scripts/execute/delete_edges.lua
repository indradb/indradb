local trans = transaction();
local outbound = vertex("foo");
trans:create_vertex(outbound);
local inbound = vertex("foo");
trans:create_vertex(inbound);
local key = EdgeKey.new(outbound.id, "baz", inbound.id);
trans:create_edge(key);
trans:delete_edges(EdgeQuery.edges({key}));

local edges = trans:get_edges(EdgeQuery.edges({key}));
assert(#edges == 0);
