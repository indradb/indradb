local queries = require("queries");
local id = create_vertex("foo");
set_vertex(id, "bar");
local vertices = get_vertices(queries.vertex(id).query);
assert(table.getn(vertices) == 1);
assert(vertices[1].type == "bar");
delete_vertices(queries.vertex(id).query);
