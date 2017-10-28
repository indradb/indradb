local queries = require("queries");
local id = create_vertex(trans, "foo");
delete_vertices(trans, queries.VertexQuery.vertex(id));
local vertices = get_vertices(trans, queries.VertexQuery.vertex(id));
assert(#vertices == 0);
