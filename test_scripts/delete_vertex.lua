local id = create_vertex("foo");

function test_delete_vertex()
    delete_vertex(id);
    get_vertex(id);
end

local status, err = pcall(test_delete_vertex);
assert(status == false);
assert(string.find(err, "VertexNotFound"));
