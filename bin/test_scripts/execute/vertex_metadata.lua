local trans = transaction();
local v = vertex("foo");
trans:create_vertex(v);
local q = VertexQuery.vertices({v.id});

trans:set_vertex_metadata(q, "script-test-vertex", {foo={true, false}});

local val = trans:get_vertex_metadata(q, "script-test-vertex");
assert(#val == 1);
assert(val[1].id == v.id);
assert(val[1].value.foo[1] == true);
assert(val[1].value.foo[2] == false);

trans:delete_vertex_metadata(q, "script-test-vertex");
local val = trans:get_vertex_metadata(q, "script-test-vertex");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
