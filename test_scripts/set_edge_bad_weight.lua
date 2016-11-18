local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");

function test_set_edge_weight_out_of_range()
    set_edge(outbound_id, "purchased", inbound_id, -2);
end

function test_set_edge_bad_weight()
    set_edge(outbound_id, "purchased", inbound_id, "foo");
end

local status, err = pcall(test_set_edge_weight_out_of_range);
assert(status == false);
assert(string.find(err, "OutOfRange"));
local status, err = pcall(test_set_edge_bad_weight);
assert(status == false);
assert(string.find(err, "number expected, got string"));
