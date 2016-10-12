outbound_id = create_vertex("foo", "{}");
inbound_id = create_vertex("bar", "{}");
set_edge(outbound_id, "baz", inbound_id, 0.5, "{}");
edges = get_edge_range(outbound_id, "baz", 0, 5);

assert(table.getn(edges, 1));
assert(edges[1].outbound_id == outbound_id);
assert(edges[1].type == "baz");
assert(edges[1].inbound_id == inbound_id);
assert(edges[1].weight == 0.5);
assert(edges[1].properties == "{}");
