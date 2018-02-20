local trans = transaction();
local id = trans:create_vertex("foo");
trans:delete_vertices(VertexQuery.vertices({id}));
local vertices = trans:get_vertices(VertexQuery.vertices({id}));
assert(#vertices == 0);
