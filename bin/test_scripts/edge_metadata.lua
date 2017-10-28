local queries = require("queries");

local outbound_id = create_vertex(trans, "foo");
local inbound_id = create_vertex(trans, "bar");
create_edge(trans, queries.EdgeKey.new(outbound_id, "baz", inbound_id), 0.5);

local q = queries.EdgeQuery.edge(outbound_id, "baz", inbound_id);
set_edge_metadata(trans, q, "script-test-edge", {foo={true, false}});
local val = get_edge_metadata(trans, q, "script-test-edge");
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

delete_edge_metadata(trans, q, "script-test-edge");
local val = get_edge_metadata(trans, q, "script-test-edge");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
