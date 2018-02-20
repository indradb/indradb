local queries = require("../shared/queries");

function map(vertex)
    local trans = transaction();
    trans:set_vertex_metadata(queries.VertexQuery.vertices({vertex.id}), "foo", "bar");

end

function reduce(accumulator, value)
    return nil
end

return { map=map, reduce=reduce }
