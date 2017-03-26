local queries = require("queries");

local id = create_vertex("foo");
local q = vertex(id):outbound_edges("purchased", "bar", 10, 10);

function test_get_edge_range_bad_high()
    get_edges(q.query);
end

local status, err = pcall(test_get_edge_range_bad_high)
assert(status == false);
assert(string.find(err, "Expected edge query table"));
