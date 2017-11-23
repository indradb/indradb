local queries = require("queries");
local id = create_vertex(trans, "foo");
delete_vertices(trans, queries.VertexQuery.vertices({id}));
local vertices = get_vertices(trans, queries.VertexQuery.vertices({id}));
assert(#vertices == 0);
