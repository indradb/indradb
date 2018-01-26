local queries = require("queries");

local outbound_id = trans:create_vertex("foo");
local inbound_id = trans:create_vertex("bar");
local key = queries.EdgeKey.new(outbound_id, "baz", inbound_id);
trans:create_edge(key, 0.5);
trans:delete_edges(queries.EdgeQuery.edges({key}));

local edges = trans:get_edges(queries.EdgeQuery.edges({key}));
assert(#edges == 0);
