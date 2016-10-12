x = transaction({
  get_edge_range("3", "purchased", "3", -1)
});

assert(table.getn(x) == 1);
assert(x[1]._type == "limit_out_of_range");

transaction({
  get_edge_range("3", "purchased", 3, "foo")
});

-- error: runtime
