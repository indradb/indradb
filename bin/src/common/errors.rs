// TODO: remove this once error-chain fixes it
#![allow(renamed_and_removed_lints)]

use std::io;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IoError(io::Error);
    }
}
