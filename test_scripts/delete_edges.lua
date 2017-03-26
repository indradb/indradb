local queries = require("queries");

local outbound_id = create_vertex("foo");
local inbound_id = create_vertex("bar");
create_edge(outbound_id, "baz", inbound_id, 0.5);
delete_edges(queries.edge(outbound_id, "baz", inbound_id).query);

local edges = get_edges(queries.edge(outbound_id, "baz", inbound_id).query);
assert(table.getn(edges) == 0);
