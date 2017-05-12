local VertexQuery = {}
VertexQuery.__index = VertexQuery

local EdgeQuery = {}
EdgeQuery.__index = EdgeQuery

function all_vertices(start_id, limit)
    local self = setmetatable({}, VertexQuery)
    self.query = {type="all", start_id=start_id, limit=limit}
    return self
end

function vertex(id)
    local self = setmetatable({}, VertexQuery)
    self.query = {type="vertex", id=id}
    return self
end

function vertices(ids)
    local self = setmetatable({}, VertexQuery)
    self.query = {type="vertices", ids=ids}
    return self
end

function vertex_query_pipe(edge_query, converter, limit)
    local self = setmetatable({}, VertexQuery)
    self.query = {type="pipe", edge_query=edge_query, converter=converter, limit=limit}
    return self
end

function VertexQuery:outbound_edges(t, high, low, limit)
    return edge_query_pipe(self.query, "outbound", t, high, low, limit)
end

function VertexQuery:inbound_edges(t, high, low, limit)
    return edge_query_pipe(self.query, "inbound", t, high, low, limit)
end

function edge(outbound_id, t, inbound_id)
    local self = setmetatable({}, EdgeQuery)
    self.query = {type="edge", key={outbound_id=outbound_id, t=t, inbound_id=inbound_id}}
    return self
end

function edges(keys)
    local self = setmetatable({}, EdgeQuery)
    self.query = {keys=keys}
    return self
end

function edge_query_pipe(vertex_query, converter, type_filter, high_filter, low_filter, limit)
    local self = setmetatable({}, EdgeQuery)
    self.query = {
        type="pipe",
        vertex_query=vertex_query,
        converter=converter,
        type_filter=type_filter,
        high_filter=high_filter,
        low_filter=low_filter,
        limit=limit
    }
    return self
end

function EdgeQuery:outbound_vertices(limit)
    return vertex_query_pipe(self.query, "outbound", limit)
end

function EdgeQuery:inbound_vertices(limit)
    return vertex_query_pipe(self.query, "inbound", limit)
end

return {
    all_vertices=all_vertices,
    vertex=vertex,
    vertices=vertices,
    all_edges=all_edges,
    edge=edge,
    edges=edges,
    json_null={"__braid_json_null"}
}
