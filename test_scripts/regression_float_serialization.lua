-- REGRESSION: There was a bug where the wrong item in the lua stack was 
-- serialized. This broke serialization of primitive types, including floats
-- which are tested here via global metadata.
set_global_metadata("script-test-regression-float-serialization", 3.14);
local val = get_global_metadata("script-test-regression-float-serialization");
assert(val == 3.14);
delete_global_metadata("script-test-regression-float-serialization");
