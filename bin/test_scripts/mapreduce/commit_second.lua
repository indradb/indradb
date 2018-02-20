local queries = require("../shared/queries");

function map(vertex)
    local trans = transaction();
    return trans:get_vertex_metadata(queries.VertexQuery.vertices({vertex.id}), "foo");
end

function reduce(accumulator, metadata)
    accumulator = accumulator or 0

    for _, value in pairs(metadata) do
        assert(value.value == "bar");
        accumulator = accumulator + 1
    end

    return accumulator
end

return { map=map, reduce=reduce }
