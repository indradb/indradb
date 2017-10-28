local queries = require("queries");

local id = create_vertex(trans, "foo");
local q = queries.VertexQuery.vertex(id);

set_vertex_metadata(trans, q, "script-test-vertex", {foo={true, false}});

local val = get_vertex_metadata(trans, q, "script-test-vertex");
assert(val[id].foo[1] == true);
assert(val[id].foo[2] == false);

delete_vertex_metadata(trans, q, "script-test-vertex");
local val = get_vertex_metadata(trans, q, "script-test-vertex");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
