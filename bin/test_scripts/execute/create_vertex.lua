local queries = require("../shared/queries");
local trans = transaction();
local id = trans:create_vertex("foo");
local vertices = trans:get_vertices(queries.VertexQuery.vertices({id}));
assert(vertices[1].id == id);
assert(vertices[1].type == "foo");
