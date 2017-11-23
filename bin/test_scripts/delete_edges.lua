local queries = require("queries");

local outbound_id = create_vertex(trans, "foo");
local inbound_id = create_vertex(trans, "bar");
local key = queries.EdgeKey.new(outbound_id, "baz", inbound_id);
create_edge(trans, key, 0.5);
delete_edges(trans, queries.EdgeQuery.edges({key}));

local edges = get_edges(trans, queries.EdgeQuery.edges({key}));
assert(#edges == 0);
