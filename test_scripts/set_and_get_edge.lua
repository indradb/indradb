local queries = require("queries");

local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
create_edge(outbound_id, "baz", inbound_id, 0.5);
local e = get_edges(queries.edge(outbound_id, "baz", inbound_id).query);
assert(table.getn(e) == 1);
assert(e[1].key.outbound_id == outbound_id);
assert(e[1].key.type == "baz");
assert(e[1].key.inbound_id == inbound_id);
