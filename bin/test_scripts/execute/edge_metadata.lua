local queries = require("../shared/queries");

local outbound_id = trans:create_vertex("foo");
local inbound_id = trans:create_vertex("bar");
local key = queries.EdgeKey.new(outbound_id, "baz", inbound_id);
trans:create_edge(key, 0.5);

local q = queries.EdgeQuery.edges({key});
trans:set_edge_metadata(q, "script-test-edge", {foo={true, false}});
local val = trans:get_edge_metadata(q, "script-test-edge");
local already_iterated = false;

for k, v in pairs(val) do
    if already_iterated then
        error("More than one key found in the edge metadata value");
    end

    assert(k.outbound_id == outbound_id);
    assert(k.type == "baz");
    assert(k.inbound_id == inbound_id);
    assert(v.foo[1] == true);
    assert(v.foo[2] == false);
    already_iterated = true;
end

trans:delete_edge_metadata(q, "script-test-edge");
local val = trans:get_edge_metadata(q, "script-test-edge");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
