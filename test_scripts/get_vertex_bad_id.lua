function test_get_vertex_bad_id()
    get_vertex("asdasd");
end

local status, err = pcall(test_get_vertex_bad_id)
assert(status == false);
assert(string.find(err, "Expected uuid as string"));
