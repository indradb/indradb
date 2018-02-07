trans:set_account_metadata(account_id, "script-test-account", {foo={true, false}});
local val = trans:get_account_metadata(account_id, "script-test-account");
assert(val.foo[1] == true);
assert(val.foo[2] == false);
trans:delete_account_metadata(account_id, "script-test-account");

function test_get_deleted_account_metadata()
    trans:get_account_metadata(account_id, "script-test-account");
end

local status, err = pcall(test_get_deleted_account_metadata);
assert(status == false);
assert(tostring(err) == "runtime error: Metadata does not exist");
