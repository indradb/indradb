local trans = transaction();
local id = trans:create_vertex_from_type("foo");
local q = VertexQuery.vertices({id}):outbound_edges("purchased", "bar", 10, 10);

function test_get_edge_range_bad_high()
    trans:get_edges(q);
end

local status, err = pcall(test_get_edge_range_bad_high)
assert(status == false);
assert(tostring(err) == "error converting Lua non-number to datetime");
