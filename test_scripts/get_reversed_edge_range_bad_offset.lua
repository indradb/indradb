local id = create_vertex("foo");

function test_get_reversed_edge_range_offset_out_of_range()
    get_reversed_edge_range(id, "purchased", -1, 10);
end

function test_get_reversed_edge_range_bad_offset()
    get_reversed_edge_range(id, "purchased", "foo", 10);
end

local status, err = pcall(test_get_reversed_edge_range_offset_out_of_range)
assert(status == false);
assert(string.find(err, "Offset cannot be negative"));
local status, err = pcall(test_get_reversed_edge_range_bad_offset)
assert(status == false);
assert(string.find(err, "number expected, got string"));
