local trans = transaction();

function create_vertex()
    return trans:create_vertex("foo");
end

local id = create_vertex();
trans:commit();

local status, err = pcall(create_vertex);
assert(status == false);
assert(tostring(err) == "runtime error: The transaction has already finished");

return id;
