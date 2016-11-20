local id = create_vertex("foo");
set_vertex_metadata(id, "script-test-vertex", {foo={true, false}});
local val = get_vertex_metadata(id, "script-test-vertex");
assert(val.foo["1"] == true);
assert(val.foo["2"] == false);
delete_vertex_metadata(id, "script-test-vertex");

function test_get_deleted_vertex_metadata()
    get_vertex_metadata(id, "script-test-vertex");
end

local status, err = pcall(test_get_deleted_vertex_metadata);
assert(status == false);
assert(string.find(err, "MetadataNotFound"));
