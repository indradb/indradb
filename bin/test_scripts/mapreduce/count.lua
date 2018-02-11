function map(vertex)
    return arg
end

function reduce(first, second)
    return first + second
end

return { map=map, reduce=reduce }
