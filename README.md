# braid

[![Build Status](https://travis-ci.org/braidery/lib.svg?branch=master)](https://travis-ci.org/braidery/lib) [rustdoc](https://braidery.github.io/api-doc/braid/index.html)

This is the braid library. Most users can use the [server](https://github.com/braidery/braid), which provides higher-level methods for working with braid databases. Linking directly against the library would be necessary if you want to make a new datastore implementation, or if you want to work at a lower-level than the server affords.

### Creating a custom datastore

To implement a custom datastore, you need to implement the [Datastore](https://braidery.github.io/api-doc/braid/trait.Datastore.html) and [Transaction](https://braidery.github.io/api-doc/braid/trait.Transaction.html) traits. See the [postgres](https://github.com/braidery/lib/blob/develop/src/pg/datastore.rs) and [rocksdb](https://github.com/braidery/lib/blob/develop/src/rdb/datastore.rs) datastores as examples.

To help you get off the ground faster, we've defined some standard tests that can execute against any datastore and check for common bugs and regressions. See the [postgres datastore tests](https://github.com/braidery/lib/blob/develop/src/pg/tests.rs) for an example.
