x = transaction({
  set_edge(3, "purchased", 5, -2, "{}")
});

assert(table.getn(x) == 1);
assert(x[1]._type == "weight_out_of_range");

transaction({
  set_edge(3, "purchased", 5, "foo", "{}")
});

-- error: runtime
