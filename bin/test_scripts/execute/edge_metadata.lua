local trans = transaction();
local outbound = vertex("foo");
trans:create_vertex(outbound);
local inbound = vertex("foo");
trans:create_vertex(inbound);
local key = EdgeKey.new(outbound.id, "baz", inbound.id);
trans:create_edge(key);

local q = EdgeQuery.edges({key});
trans:set_edge_metadata(q, "script-test-edge", {foo={true, false}});
local val = trans:get_edge_metadata(q, "script-test-edge");

assert(#val == 1);
assert(val[1].key.outbound_id == outbound.id);
assert(val[1].key.type == "baz");
assert(val[1].key.inbound_id == inbound.id);
assert(val[1].value.foo[1] == true);
assert(val[1].value.foo[2] == false);

trans:delete_edge_metadata(q, "script-test-edge");
local val = trans:get_edge_metadata(q, "script-test-edge");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
