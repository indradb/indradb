id = create_vertex("foo", "{}");
set_vertex(id, "bar", "{}");
v = get_vertex(id);
assert(v.type == "bar");
delete_vertex(id);
