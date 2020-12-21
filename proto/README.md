# IndraDB protos [![Docs](https://docs.rs/indradb-proto/badge.svg)](https://docs.rs/indradb-proto)

This crate contains:

1) The cap'n proto schema definition
2) The auto-generated cap'n proto code
3) Some utility functions for making it more convenient to convert between IndraDB types and their cap'n proto equivalents.

This crate is useful if you're wanting to interface with IndraDB from rust, without using the library (i.e. with a client/server relationship.) For exapmple, see the [wikipedia dataset inserter](https://github.com/indradb/wikipedia-example/tree/master/inserter), which uses rust to maximize performance.
