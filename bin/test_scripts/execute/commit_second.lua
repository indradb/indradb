local queries = require("../shared/queries");
local vertices = trans:get_vertices(queries.VertexQuery.vertices({arg}));
assert(#vertices == 1);
assert(vertices[1].id == arg);
