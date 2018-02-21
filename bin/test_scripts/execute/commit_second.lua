local trans = transaction();
local vertices = trans:get_vertices(VertexQuery.vertices({arg}));
assert(#vertices == 1);
assert(vertices[1].id == arg);
