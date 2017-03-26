local VertexQuery = {}
VertexQuery.__index = VertexQuery

local EdgeQuery = {}
EdgeQuery.__index = EdgeQuery

function all_vertices(from_id, limit)
    local self = setmetatable({}, VertexQuery)
    self.query = {all={from_id, limit}}
    return self
end

function vertex(id)
    local self = setmetatable({}, VertexQuery)
    self.query = {vertex=id}
    return self
end

function vertices(ids)
    local self = setmetatable({}, VertexQuery)
    self.query = {vertices=ids}
    return self
end

function vertex_query_pipe(previous_query, type_converter, limit)
    local self = setmetatable({}, VertexQuery)
    self.query = {pipe={previous_query, type_converter, limit}}
    return self
end

function VertexQuery:outbound_edges(t, high, low, limit)
    return edge_query_pipe(self.query, "outbound", t, high, low, limit)
end

function VertexQuery:inbound_edges(t, high, low, limit)
    return edge_query_pipe(self.query, "inbound", t, high, low, limit)
end

function all_edges(t, high, low, limit)
    local self = setmetatable({}, EdgeQuery)
    self.query = {all={t, high, low, limit}}
    return self
end

function edge(outbound_id, t, inbound_id)
    local self = setmetatable({}, EdgeQuery)
    self.query = {edge={outbound_id, t, inbound_id}}
    return self
end

function edges(edges)
    local self = setmetatable({}, EdgeQuery)
    self.query = {edges=edges}
    return self
end

function edge_query_pipe(previous_query, type_converter, t, high, low, limit)
    local self = setmetatable({}, EdgeQuery)
    self.query = {pipe={previous_query, type_converter, t, high, low, limit}}
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
