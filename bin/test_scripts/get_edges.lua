local queries = require("queries");

local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
create_edge(outbound_id, "baz", inbound_id, 0.5);

local edges = get_edges(queries.vertex(outbound_id):outbound_edges("baz", queries.json_null, queries.json_null, 10).query);
assert(table.getn(edges) == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);
assert(edges[1].weight == 0.5);

local edges = get_edges(queries.vertex(outbound_id):outbound_edges(queries.json_null, queries.json_null, queries.json_null, 10).query);
assert(table.getn(edges) == 1);
assert(edges[1].key.outbound_id == outbound_id);
assert(edges[1].key.type == "baz");
assert(edges[1].key.inbound_id == inbound_id);
assert(edges[1].weight == 0.5);
