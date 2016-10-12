edges = get_edge_time_range("3", "purchased", 10, -1, 10);
assert(table.getn(edges) == 1);
get_edge_time_range("3", "purchased", 10, -1, 10);
-- error: runtime
