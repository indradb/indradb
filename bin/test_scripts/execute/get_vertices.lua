local trans = transaction();

-- Create some sample data
local v = {
    vertex("foo"),
    vertex("foo"),
    vertex("foo"),
    vertex("foo"),
    vertex("foo")  
};

for _, sv in ipairs(v) do
    trans:create_vertex(sv);
end

trans:create_edge(EdgeKey.new(v[1].id, "bar", v[2].id));
trans:create_edge(EdgeKey.new(v[2].id, "bar", v[3].id));
trans:create_edge(EdgeKey.new(v[3].id, "bar", v[4].id));
trans:create_edge(EdgeKey.new(v[4].id, "bar", v[5].id));

function check_vertices(vertices, expected_count, required_vertex_ids)
    if expected_count ~= nil then
        assert(#vertices == expected_count);
    end

    for _, vertex in pairs(vertices) do
        if required_vertex_ids[vertex.id] ~= nil then
            assert(vertex.type == "foo")
        end

        required_vertex_ids[vertex.id] = nil
    end

    for required_vertex_id, _ in pairs(required_vertex_ids) do
        error("Not all of the required vertex IDs were found in querying: " .. required_vertex_id)
    end
end

-- Ensure we can get all of the vertices
local vertices = trans:get_vertices(VertexQuery.all("00000000-0000-0000-0000-000000000000", 10000));
check_vertices(vertices, nil, {[v[1].id]=true, [v[2].id]=true, [v[3].id]=true, [v[4].id]=true, [v[5].id]=true});

-- Ensure we can get a specific set of vertices
local vertices = trans:get_vertices(VertexQuery.vertices({v[1].id, v[2].id, v[3].id}));
check_vertices(vertices, 3, {[v[1].id]=true, [v[2].id]=true, [v[3].id]=true});

-- Ensure we can do a piped query
local query = VertexQuery.vertices({v[1].id})
    :outbound_edges("bar", nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1);
local vertices = trans:get_vertices(query);
check_vertices(vertices, 1, {[v[5].id]=true});
