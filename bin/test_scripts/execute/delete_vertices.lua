local trans = transaction();
local v = vertex("foo");
trans:create_vertex(v);
trans:delete_vertices(VertexQuery.vertices({v.id}));
local vertices = trans:get_vertices(VertexQuery.vertices({v.id}));
assert(#vertices == 0);
