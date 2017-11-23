local queries = require("queries");

local outbound_id = create_vertex(trans, "foo");
local inbound_id = create_vertex(trans, "bar");
create_edge(trans, queries.EdgeKey.new(outbound_id, "baz", inbound_id), 0.5);

local count = get_edge_count(trans, queries.VertexQuery.vertices({outbound_id}):outbound_edges("baz", nil, nil, 1000));
assert(count == 1);

local count = get_edge_count(trans, queries.VertexQuery.vertices({outbound_id}):outbound_edges(nil, nil, nil, 1000));
assert(count == 1);
