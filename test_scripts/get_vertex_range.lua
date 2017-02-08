local first_id = create_vertex("foo");
local second_id = create_vertex("bar");

local vertices = get_vertex_range(0, 2);
assert(table.getn(vertices) == 2);
assert(vertices[1].id == first_id or vertices[1].id == second_id);
assert(vertices[2].id == first_id or vertices[2].id == second_id);

local vertices = get_vertex_range(0, 1);
assert(table.getn(vertices) == 1);
assert(vertices[1].id == first_id or vertices[1].id == second_id);

local vertices = get_vertex_range(1, 1);
assert(table.getn(vertices) == 1);
assert(vertices[1].id == first_id or vertices[1].id == second_id);
