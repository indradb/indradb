local trans = transaction();
local id = trans:create_vertex_from_type("foo");
local vertices = trans:get_vertices(VertexQuery.vertices({id}));
assert(vertices[1].id == id);
assert(vertices[1].type == "foo");
