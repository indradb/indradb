x = transaction({
  get_edge_time_range("3", "purchased", -1, 10, 10)
});

assert(table.getn(x) == 1);
assert(x[1]._type == "edges");
assert(table.getn(x[1].edges) == 1);

transaction({
  get_edge_time_range("3", "purchased", "foo", 10, 10)
});

-- error: runtime
