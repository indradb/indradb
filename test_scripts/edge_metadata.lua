local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
set_edge(outbound_id, "baz", inbound_id, 0.5);

set_edge_metadata(outbound_id, "baz", inbound_id, "script-test-edge", {foo={true, false}});
local val = get_edge_metadata(outbound_id, "baz", inbound_id, "script-test-edge");
assert(val.foo[1] == true);
assert(val.foo[2] == false);
delete_edge_metadata(outbound_id, "baz", inbound_id, "script-test-edge");

function test_get_deleted_edge_metadata()
    get_edge_metadata(outbound_id, "baz", inbound_id, "script-test-edge");
end

local status, err = pcall(test_get_deleted_edge_metadata);
assert(status == false);
assert(string.find(err, "MetadataNotFound"));
