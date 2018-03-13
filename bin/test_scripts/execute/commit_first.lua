local trans = transaction();
local v = vertex("foo");
trans:create_vertex(v);
return v
