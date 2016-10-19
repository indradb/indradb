set_metadata("", "script-test-global", "{}");
assert(get_metadata("", "script-test-global") == "{}");
delete_metadata("", "script-test-global");
get_metadata("", "script-test-global");
-- error: runtime
