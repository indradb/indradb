local queries = require("queries");

local outbound_id = create_vertex(trans, "foo");
local inbound_id = create_vertex(trans, "bar");
create_edge(trans, queries.EdgeKey.new(outbound_id, "baz", inbound_id), 0.5);
local e = get_edges(trans, queries.EdgeQuery.edge(outbound_id, "baz", inbound_id));
assert(#e == 1);
assert(e[1].key.outbound_id == outbound_id);
assert(e[1].key.type == "baz");
assert(e[1].key.inbound_id == inbound_id);
