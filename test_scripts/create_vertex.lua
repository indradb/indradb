local queries = require("queries");
local id = create_vertex("foo");
local vertices = get_vertices(queries.vertex(id).query);
assert(vertices[1].id == id);
assert(vertices[1].type == "foo");
