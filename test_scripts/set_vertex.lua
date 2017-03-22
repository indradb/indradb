local queries = require("queries");
local id = create_vertex("foo");
set_vertex(id, "bar");
local vertices = get_vertices(queries.VertexQuery.vertex(id).query);
assert(table.getn(vertices) == 1);
assert(vertices[1].type == "bar");
delete_vertex(id);
