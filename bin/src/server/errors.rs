use indradb;
use uuid;

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
    }
}
