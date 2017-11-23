local queries = require("queries");
local id = create_vertex(trans, "foo");
local vertices = get_vertices(trans, queries.VertexQuery.vertices({id}));
assert(vertices[1].id == id);
assert(vertices[1].type == "foo");
