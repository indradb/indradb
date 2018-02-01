local queries = require("../shared/queries");

local id = trans:create_vertex("foo");

local q1 = queries.VertexQuery.vertices({id}):outbound_edges("purchased", nil, nil, -1);
function test_get_edge_range_limit_out_of_range()
    trans:get_edges(q1);
end

local q2 = queries.VertexQuery.vertices({id}):outbound_edges("purchased", nil, nil, "foo");
function test_get_edge_range_bad_limit()
    trans:get_edges(q2);
end

local status, err = pcall(test_get_edge_range_limit_out_of_range);
assert(status == false);
assert(tostring(err) == "error converting Lua integer to limit (value is below 0)");
local status, err = pcall(test_get_edge_range_bad_limit);
assert(status == false);
assert(tostring(err) == "error converting Lua string to integer");
