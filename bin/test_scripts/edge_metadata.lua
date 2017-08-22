local queries = require("queries");

local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
create_edge(outbound_id, "baz", inbound_id, 0.5);

local q = queries.edge(outbound_id, "baz", inbound_id);
set_edge_metadata(q.query, "script-test-edge", {foo={true, false}});
local val = get_edge_metadata(q.query, "script-test-edge");
local already_iterated = false;

for k, v in pairs(val) do
    if already_iterated then
        error("More than one key found in the edge metadata value");
    end

    assert(k[1] == outbound_id);
    assert(k[2] == "baz");
    assert(k[3] == inbound_id);
    assert(v.foo[1] == true);
    assert(v.foo[2] == false);
    already_iterated = true;
end

delete_edge_metadata(q.query, "script-test-edge");
local val = get_edge_metadata(q.query, "script-test-edge");

for id, value in pairs(val) do
    error("Unexpected item returned after deleting metadata: " .. id .. "->" .. value)
end
