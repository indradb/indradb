local trans = transaction();

-- Create some sample data
local id_1 = trans:create_vertex("foo");
local id_2 = trans:create_vertex("foo");
local id_3 = trans:create_vertex("foo");
local id_4 = trans:create_vertex("foo");
local id_5 = trans:create_vertex("foo");
trans:create_edge(EdgeKey.new(id_1, "bar", id_2));
trans:create_edge(EdgeKey.new(id_2, "bar", id_3));
trans:create_edge(EdgeKey.new(id_3, "bar", id_4));
trans:create_edge(EdgeKey.new(id_4, "bar", id_5));

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
check_vertices(vertices, nil, {[id_1]=true, [id_2]=true, [id_3]=true, [id_4]=true, [id_5]=true});

-- Ensure we can get a specific set of vertices
local vertices = trans:get_vertices(VertexQuery.vertices({id_1, id_2, id_3}));
check_vertices(vertices, 3, {[id_1]=true, [id_2]=true, [id_3]=true});

-- Ensure we can do a piped query
local query = VertexQuery.vertices({id_1})
    :outbound_edges("bar", nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1)
    :outbound_edges(nil, nil, nil, 1):inbound_vertices(1);
local vertices = trans:get_vertices(query);
check_vertices(vertices, 1, {[id_5]=true});
