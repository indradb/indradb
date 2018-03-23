local trans = transaction();

-- Create some sample data
local v = {
    trans:create_vertex_from_type("foo"),
    trans:create_vertex_from_type("foo"),
    trans:create_vertex_from_type("foo"),
    trans:create_vertex_from_type("foo"),
    trans:create_vertex_from_type("foo")
};

trans:create_edge(EdgeKey.new(v[1], "bar", v[2]));
trans:create_edge(EdgeKey.new(v[2], "bar", v[3]));
trans:create_edge(EdgeKey.new(v[3], "bar", v[4]));
trans:create_edge(EdgeKey.new(v[4], "bar", v[5]));

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
check_vertices(vertices, nil, {[v[1]]=true, [v[2]]=true, [v[3]]=true, [v[4]]=true, [v[5]]=true});

-- Ensure we can get a specific set of vertices
local vertices = trans:get_vertices(VertexQuery.vertices({v[1], v[2], v[3]}));
check_vertices(vertices, 3, {[v[1]]=true, [v[2]]=true, [v[3]]=true});

-- Ensure we can do a piped query
local query = VertexQuery.vertices({v[1]})
    :outbound_edges("bar", nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1);
local vertices = trans:get_vertices(query);
check_vertices(vertices, 1, {[v[5]]=true});
