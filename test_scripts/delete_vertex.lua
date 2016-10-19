id = create_vertex("foo", "{}");
delete_vertex(id);
get_vertex(id);
-- error: runtime
