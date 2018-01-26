local queries = require("queries");
local id = trans:create_vertex("foo");
trans:delete_vertices(queries.VertexQuery.vertices({id}));
local vertices = trans:get_vertices(queries.VertexQuery.vertices({id}));
assert(#vertices == 0);
