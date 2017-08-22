local queries = require("queries");

local id = create_vertex("foo");

local q1 = queries.vertex(id):outbound_edges("purchased", "", "", -1);
function test_get_edge_range_limit_out_of_range()
    get_edges(q1.query);
end

local q2 = queries.vertex(id):outbound_edges("purchased", "", "", "foo");
function test_get_edge_range_bad_limit()
    get_edges(q2.query);
end

local status, err = pcall(test_get_edge_range_limit_out_of_range)
assert(status == false);
assert(string.find(err, "Expected edge query table"));
local status, err = pcall(test_get_edge_range_bad_limit)
assert(status == false);
assert(string.find(err, "Expected edge query table"));
