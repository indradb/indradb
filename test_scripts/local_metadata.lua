set_metadata("__acount_id__", "script-test-local", "{}");
assert(get_metadata("__acount_id__", "script-test-local") == "{}");
delete_metadata("__acount_id__", "script-test-local");
get_metadata("__acount_id__", "script-test-local");
-- error: runtime
