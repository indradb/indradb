local queries = require("queries");

local outbound_id = create_vertex(trans, "foo");
local inbound_id = create_vertex(trans, "bar");
local key = queries.EdgeKey.new(outbound_id, "baz", inbound_id);
create_edge(trans, key);
local e = get_edges(trans, queries.EdgeQuery.edges({key}));
assert(#e == 1);
assert(e[1].key.outbound_id == outbound_id);
assert(e[1].key.type == "baz");
assert(e[1].key.inbound_id == inbound_id);
