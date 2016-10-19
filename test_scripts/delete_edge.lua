outbound_id = create_vertex("foo", "{}");
inbound_id = create_vertex("bar", "{}");

set_edge(outbound_id, "baz", inbound_id, 0.5, "{}");
delete_edge(outbound_id, "baz", inbound_id);
get_edge(outbound_id, "baz", inbound_id);
-- error: runtime
