function map(vertex)
    return arg
end

function reduce(accumulator, value)
    return (accumulator or 1.0) + value
end

return { map=map, reduce=reduce }
