function map(vertex)
    local trans = transaction();
    trans:set_vertex_metadata(VertexQuery.vertices({vertex.id}), "foo", "bar");
end

function reduce(accumulator, value)
    return nil
end

return { map=map, reduce=reduce }
