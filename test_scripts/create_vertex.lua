id = create_vertex("foo", "{}");
v = get_vertex(id);
assert(v.id == id);
assert(v.type == "foo");
assert(v.properties == "{}");
