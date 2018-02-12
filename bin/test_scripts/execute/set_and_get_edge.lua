local queries = require("../shared/queries");

local outbound_id = trans:create_vertex("foo");
local inbound_id = trans:create_vertex("bar");
local key = queries.EdgeKey.new(outbound_id, "baz", inbound_id);
trans:create_edge(key);
local e = trans:get_edges(queries.EdgeQuery.edges({key}));
assert(#e == 1);
assert(e[1].key.outbound_id == outbound_id);
assert(e[1].key.type == "baz");
assert(e[1].key.inbound_id == inbound_id);
