local id = create_vertex("foo");

function test_get_edge_range_limit_out_of_range()
    get_edge_range(id, "purchased", "", "", -1);
end

function test_get_edge_range_bad_limit()
    get_edge_range(id, "purchased", "", "", "foo");
end

local status, err = pcall(test_get_edge_range_limit_out_of_range)
assert(status == false);
assert(string.find(err, "Limit cannot be negative"));
local status, err = pcall(test_get_edge_range_bad_limit)
assert(status == false);
assert(string.find(err, "number expected, got string"));