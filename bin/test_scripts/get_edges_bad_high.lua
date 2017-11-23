local queries = require("queries");

local id = create_vertex(trans, "foo");
local q = queries.VertexQuery.vertices({id}):outbound_edges("purchased", "bar", 10, 10);

function test_get_edge_range_bad_high()
    get_edges(trans, q);
end

local status, err = pcall(test_get_edge_range_bad_high)
assert(status == false);
assert(tostring(err) == "error converting Lua non-number to datetime");
