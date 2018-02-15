local queries = require("../shared/queries");
local trans = transaction();
local id = trans:create_vertex("foo");
trans:delete_vertices(queries.VertexQuery.vertices({id}));
local vertices = trans:get_vertices(queries.VertexQuery.vertices({id}));
assert(#vertices == 0);
