use std::io;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IoError(io::Error);
    }
}
