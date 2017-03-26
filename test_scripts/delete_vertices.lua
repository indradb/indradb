local queries = require("queries");
local id = create_vertex("foo");
delete_vertices(queries.vertex(id).query);
local vertices = get_vertices(queries.vertex(id).query);
assert(table.getn(vertices), 0);
