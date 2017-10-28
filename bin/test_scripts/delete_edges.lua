local queries = require("queries");

local outbound_id = create_vertex(trans, "foo");
local inbound_id = create_vertex(trans, "bar");
create_edge(trans, queries.EdgeKey.new(outbound_id, "baz", inbound_id), 0.5);
delete_edges(trans, queries.EdgeQuery.edge(outbound_id, "baz", inbound_id));

local edges = get_edges(trans, queries.EdgeQuery.edge(outbound_id, "baz", inbound_id));
assert(#edges == 0);
