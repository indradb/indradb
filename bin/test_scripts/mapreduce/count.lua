-- This will actually return `arg * #vertices + 1` - so it's not an actual
-- count. But it tests the various aspects of mapreduce.

function map(vertex)
    return arg;
end

function reduce(first, second)
    return (first or 1.0) + second;
end

return { map=map, reduce=reduce }
