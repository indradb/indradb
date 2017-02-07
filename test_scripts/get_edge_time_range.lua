local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
set_edge(outbound_id, "baz", inbound_id, 0.5);

local edges = get_edge_time_range(outbound_id, "baz", "", "", 10);
assert(table.getn(edges) == 1);
assert(edges[1].outbound_id == outbound_id);
assert(edges[1].type == "baz");
assert(edges[1].inbound_id == inbound_id);

local edges = get_edge_time_range(outbound_id, "", "", "", 10);
assert(table.getn(edges) == 1);
assert(edges[1].outbound_id == outbound_id);
assert(edges[1].type == "baz");
assert(edges[1].inbound_id == inbound_id);
