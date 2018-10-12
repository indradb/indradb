// TODO: remove this once error-chain fixes it
#![allow(renamed_and_removed_lints)]

use indradb;
use serde_json;
use uuid;
use std::io;
use capnp;

// TODO: remove any unused links
error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        TransactionError(indradb::Error, indradb::ErrorKind);
        ValidationError(indradb::ValidationError, indradb::ValidationErrorKind);
    }

    foreign_links {
        UuidParseError(uuid::parser::ParseError);
        JsonError(serde_json::Error);
        IoError(io::Error);
        CapnpError(capnp::Error);
    }
}
