local trans = transaction();
local v = vertex("foo");
trans:create_vertex(v);
local vertices = trans:get_vertices(VertexQuery.vertices({v.id}));
assert(vertices[1].id == v.id);
assert(vertices[1].type == "foo");
