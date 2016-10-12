x = transaction({
  get_edge_range("3", "purchased", -1, "10")
});

assert(table.getn(x) == 1);
assert(x[1]._type == "offset_out_of_range");

transaction({
  get_edge_range("3", "purchased", "foo", 3)
});

-- error: runtime
