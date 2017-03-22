local queries = require("queries");
local id = create_vertex("foo");
delete_vertex(id);
local vertices = get_vertices(queries.VertexQuery.vertex(id).query);
assert(table.getn(vertices), 0);
