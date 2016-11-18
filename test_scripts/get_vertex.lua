local id = create_vertex("foo");
local v = get_vertex(id);
assert(v.id == id);
assert(v.type == "foo");
