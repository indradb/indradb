local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
set_edge(outbound_id, "baz", inbound_id, 0.5);
local count = get_edge_count(outbound_id, "baz");
assert(count == 1);
