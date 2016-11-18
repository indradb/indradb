local id = create_vertex("foo");
set_vertex(id, "bar");
local v = get_vertex(id);
assert(v.type == "bar");
delete_vertex(id);
