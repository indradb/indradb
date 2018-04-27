use indradb;
use serde_json;
use uuid;
use std::io;
use capnp;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        TransactionError(indradb::Error, indradb::ErrorKind);
        ValidationError(indradb::ValidationError, indradb::ValidationErrorKind);
    }

    foreign_links {
        UuidParseError(uuid::ParseError);
        JsonError(serde_json::Error);
        IoError(io::Error);
        CapnpError(capnp::Error);
    }
}
