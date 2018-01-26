local queries = require("queries");

local id = trans:create_vertex("foo");
local q = queries.VertexQuery.vertices({id});

trans:set_vertex_metadata(q, "script-test-vertex", {foo={true, false}});

local val = trans:get_vertex_metadata(q, "script-test-vertex");
assert(val[id].foo[1] == true);
assert(val[id].foo[2] == false);

trans:delete_vertex_metadata(q, "script-test-vertex");
local val = trans:get_vertex_metadata(q, "script-test-vertex");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
