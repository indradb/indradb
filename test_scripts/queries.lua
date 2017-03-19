local VertexQuery = {}
VertexQuery.__index = VertexQuery

local EdgeQuery = {}
EdgeQuery.__index = EdgeQuery

function VertexQuery.all(from_id, limit)
    local self = setmetatable({}, VertexQuery)
    self.query = {all={from_id, limit}}
    return self
end

function VertexQuery.vertex(id)
    local self = setmetatable({}, VertexQuery)
    self.query = {vertex=id}
    return self
end

function VertexQuery.vertices(ids)
    local self = setmetatable({}, VertexQuery)
    self.query = {vertices=ids}
    return self
end

function VertexQuery.pipe(previous_query, type_converter, limit)
    local self = setmetatable({}, VertexQuery)
    self.query = {pipe={previous_query, type_converter, limit}}
    return self
end

function VertexQuery.outbound_edges(self, t, high, low, limit)
    return EdgeQuery.pipe(self.query, "outbound", t, high, low, limit)
end

function VertexQuery.inbound_edges(self, t, high, low, limit)
    return EdgeQuery.pipe(self.query, "inbound", t, high, low, limit)
end

function EdgeQuery.all(t, high, low, limit)
    local self = setmetatable({}, EdgeQuery)
    self.query = {all={t, high, low, limit}}
    return self
end

function EdgeQuery.edge(outbound_id, t, inbound_id)
    local self = setmetatable({}, EdgeQuery)
    self.query = {edge={outbound_id, t, inbound_id}}
    return self
end

function EdgeQuery.edges(edges)
    local self = setmetatable({}, EdgeQuery)
    self.query = {edges=edges}
    return self
end

function EdgeQuery.pipe(previous_query, type_converter, t, high, low, limit)
    local self = setmetatable({}, EdgeQuery)
    self.query = {pipe={previous_query, type_converter, t, high, low, limit}}
    return self
end

function EdgeQuery.outbound_vertices(self, limit)
    return VertexQuery.pipe(self.query, "outbound", limit)
end

function EdgeQuery.inbound_vertices(self, limit)
    return VertexQuery.pipe(self.query, "inbound", limit)
end

return {
    VertexQuery=VertexQuery,
    EdgeQuery=EdgeQuery,
    json_null={"__braid_json_null"}
}
