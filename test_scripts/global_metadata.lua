set_global_metadata("script-test-global", "{\"foo\": true}");
local val = get_global_metadata("script-test-global");
assert(val.foo == true);
delete_global_metadata("script-test-global");

function test_get_deleted_global_metadata()
    get_global_metadata("script-test-global");
end

local status, err = pcall(test_get_deleted_global_metadata);
assert(status == false);
assert(string.find(err, "MetadataNotFound"));
