function map(vertex)
    local trans = transaction();
    return trans:get_vertex_metadata(VertexQuery.vertices({vertex.id}), "mapreduce_commit_test");
end

function value_to_count(value)
    if value == nil then
        return 0;
    elseif type(value) == "table" then
        if #value > 0 then
            assert(value[1].value == "foo");
        end

        return 1;
    else
        return value;
    end
end

function reduce(first, second)
    return value_to_count(first) + value_to_count(second);
end

return { map=map, reduce=reduce }
