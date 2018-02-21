local trans = transaction();

function foo()
end

function test_create_vertex_bad_type()
    trans:create_vertex(foo);
end

local status, err = pcall(test_create_vertex_bad_type);
assert(status == false);
assert(tostring(err) == "error converting Lua function to String (expected string or number)");
