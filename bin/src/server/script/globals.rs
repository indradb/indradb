pub const GLOBALS: &str = "
EdgeKey = {}
EdgeKey.__index = EdgeKey

VertexQuery = {}
VertexQuery.__index = VertexQuery

EdgeQuery = {}
EdgeQuery.__index = EdgeQuery

function EdgeKey.new(outbound_id, type, inbound_id)
    local self = setmetatable({}, EdgeKey)
    self.outbound_id = outbound_id
    self.type = type
    self.inbound_id = inbound_id
    return self
end

function VertexQuery.all(start_id, limit)
    local self = setmetatable({}, VertexQuery)
    self.type = \"all\"
    self.start_id = start_id
    self.limit = limit
    return self
end

function VertexQuery.vertices(ids)
    local self = setmetatable({}, VertexQuery)
    self.type = \"vertices\"
    self.ids = ids
    return self
end

function VertexQuery.pipe(edge_query, converter, limit)
    local self = setmetatable({}, VertexQuery)
    self.type = \"pipe\"
    self.edge_query = edge_query
    self.converter = converter
    self.limit = limit
    return self
end

function VertexQuery:outbound_edges(type, high, low, limit)
    return EdgeQuery.pipe(self, \"outbound\", type, high, low, limit)
end

function VertexQuery:inbound_edges(type, high, low, limit)
    return EdgeQuery.pipe(self, \"inbound\", type, high, low, limit)
end

function EdgeQuery.edges(keys)
    local self = setmetatable({}, EdgeQuery)
    self.type = \"edges\"
    self.keys = keys
    return self
end

function EdgeQuery.pipe(vertex_query, converter, type_filter, high_filter, low_filter, limit)
    local self = setmetatable({}, EdgeQuery)
    self.type = \"pipe\"
    self.vertex_query = vertex_query
    self.converter = converter
    self.type_filter = type_filter
    self.high_filter = high_filter
    self.low_filter = low_filter
    self.limit = limit
    return self
end

function EdgeQuery:outbound_vertices(limit)
    return VertexQuery.pipe(self, \"outbound\", limit)
end

function EdgeQuery:inbound_vertices(limit)
    return VertexQuery.pipe(self, \"inbound\", limit)
end
";
