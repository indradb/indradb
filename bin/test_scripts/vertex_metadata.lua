local queries = require("queries");

local id = create_vertex("foo");
local q = queries.vertex(id);

set_vertex_metadata(q.query, "script-test-vertex", {foo={true, false}});

local val = get_vertex_metadata(q.query, "script-test-vertex");
assert(val[id].foo[1] == true);
assert(val[id].foo[2] == false);

delete_vertex_metadata(q.query, "script-test-vertex");
local val = get_vertex_metadata(q.query, "script-test-vertex");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
