trans:set_global_metadata("script-test-global", {foo={true, false}});
local val = trans:get_global_metadata("script-test-global");
assert(val.foo[1] == true);
assert(val.foo[2] == false);
trans:delete_global_metadata("script-test-global");
local val = trans:get_global_metadata("script-test-global");
assert(val == nil);
