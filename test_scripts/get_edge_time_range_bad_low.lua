local id = create_vertex("foo");

function test_get_edge_time_range_bad_low()
    get_edge_time_range(id, "purchased", 10, "bar", 10);
end

local status, err = pcall(test_get_edge_time_range_bad_low)
assert(status == false);
assert(string.find(err, "Expected i64 as string"));
