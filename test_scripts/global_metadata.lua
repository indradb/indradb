set_global_metadata("script-test-global", {foo={true, false}});
local val = get_global_metadata("script-test-global");
assert(val.foo["1"] == true);
assert(val.foo["2"] == false);
delete_global_metadata("script-test-global");

function test_get_deleted_global_metadata()
    get_global_metadata("script-test-global");
end

local status, err = pcall(test_get_deleted_global_metadata);
assert(status == false);
assert(string.find(err, "MetadataNotFound"));
