function map(vertex)
    local trans = transaction();
    return trans:get_vertex_metadata(VertexQuery.vertices({vertex.id}), "foo");
end

function reduce(accumulator, value)
    accumulator = accumulator or 0

    if type(value) == "table" then
        for _, value in pairs(value) do
            assert(value.value == "bar");
            accumulator = accumulator + 1;
        end
    else
        accumulator = accumulator + value;
    end

    return accumulator;
end

return { map=map, reduce=reduce }
