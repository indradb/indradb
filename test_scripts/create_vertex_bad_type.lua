function foo()
end

function test_create_vertex_bad_type()
    create_vertex(foo);
end

local status, err = pcall(test_create_vertex_bad_type);
assert(status == false);
assert(string.find(err, "string expected, got function"));
