create_vertex("foo");
create_vertex("bar");

-- NOTE: We cannot check the actual values because other vertices are likely
-- being inserted concurrently in other script tests
local vertices = get_vertex_range("", 2);
assert(table.getn(vertices) == 2);

local vertices = get_vertex_range("", 1);
assert(table.getn(vertices) == 1);

local vertices = get_vertex_range("", 1);
assert(table.getn(vertices) == 1);
