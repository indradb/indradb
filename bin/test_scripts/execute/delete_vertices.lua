local trans = transaction();
local id = trans:create_vertex_from_type("foo");
trans:delete_vertices(VertexQuery.vertices({id}));
local vertices = trans:get_vertices(VertexQuery.vertices({id}));
assert(#vertices == 0);
