local trans = transaction();
local outbound_id = trans:create_vertex_from_type("foo");
local inbound_id = trans:create_vertex_from_type("foo");
local key = EdgeKey.new(outbound_id, "baz", inbound_id);
trans:create_edge(key);

local q = EdgeQuery.edges({key});
trans:set_edge_metadata(q, "script-test-edge", {foo={true, false}});
local val = trans:get_edge_metadata(q, "script-test-edge");

assert(#val == 1);
assert(val[1].key.outbound_id == outbound_id);
assert(val[1].key.type == "baz");
assert(val[1].key.inbound_id == inbound_id);
assert(val[1].value.foo[1] == true);
assert(val[1].value.foo[2] == false);

trans:delete_edge_metadata(q, "script-test-edge");
local val = trans:get_edge_metadata(q, "script-test-edge");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end