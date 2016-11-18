set_account_metadata(account_id, "script-test-account", "{\"foo\": true}");
local val = get_account_metadata(account_id, "script-test-account");
assert(val.foo == true);
delete_account_metadata(account_id, "script-test-account");

function test_get_deleted_account_metadata()
    get_account_metadata(account_id, "script-test-account");
end

local status, err = pcall(test_get_deleted_account_metadata);
assert(status == false);
assert(string.find(err, "MetadataNotFound"));
