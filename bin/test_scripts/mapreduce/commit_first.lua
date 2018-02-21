function map(vertex)
    local trans = transaction();
    trans:set_vertex_metadata(VertexQuery.vertices({vertex.id}), "mapreduce_commit_test", "foo");
end

function reduce(first, second)
    return nil
end

return { map=map, reduce=reduce }
