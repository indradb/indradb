local queries = require("../shared/queries");

local trans = transaction();
local id = trans:create_vertex("foo");

local q = queries.VertexQuery.vertices({id}):outbound_edges("purchased", nil, nil, "foo");
function test_get_edge_range_bad_limit()
    trans:get_edges(q);
end

local status, err = pcall(test_get_edge_range_bad_limit);
assert(status == false);
assert(tostring(err) == "error converting Lua string to integer");
