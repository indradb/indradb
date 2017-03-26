local queries = require("queries");

local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
create_edge(outbound_id, "baz", inbound_id, 0.5);

local count = get_edge_count(queries.vertex(outbound_id):outbound_edges("baz", queries.json_null, queries.json_null, 1000).query);
assert(count == 1);

local count = get_edge_count(queries.vertex(outbound_id):outbound_edges(queries.json_null, queries.json_null, queries.json_null, 1000).query);
assert(count == 1);
