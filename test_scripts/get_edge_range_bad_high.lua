local id = create_vertex("foo");

function test_get_edge_range_bad_high()
    get_edge_range(id, "purchased", "bar", 10, 10);
end

local status, err = pcall(test_get_edge_range_bad_high)
assert(status == false);
assert(string.find(err, "Expected i64 as string"));
