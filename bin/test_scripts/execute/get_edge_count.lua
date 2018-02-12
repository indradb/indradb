local queries = require("../shared/queries");

local outbound_id = trans:create_vertex("foo");
local inbound_id = trans:create_vertex("bar");
trans:create_edge(queries.EdgeKey.new(outbound_id, "baz", inbound_id));

local count = trans:get_edge_count(queries.VertexQuery.vertices({outbound_id}):outbound_edges("baz", nil, nil, 1000));
assert(count == 1);

local count = trans:get_edge_count(queries.VertexQuery.vertices({outbound_id}):outbound_edges(nil, nil, nil, 1000));
assert(count == 1);
