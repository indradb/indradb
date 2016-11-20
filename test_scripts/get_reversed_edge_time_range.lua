local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
set_edge(inbound_id, "baz", inbound_id, 0.5);

local edges = get_reversed_edge_time_range(inbound_id, "baz", "", "", 10);
assert(table.getn(edges) == 1);
assert(edges[1].outbound_id == outbound_id);
assert(edges[1].type == "baz");
assert(edges[1].inbound_id == inbound_id);
