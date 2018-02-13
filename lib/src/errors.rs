#[cfg(feature = "postgres-datastore")]
use postgres::Error as PostgresError;
#[cfg(feature = "postgres-datastore")]
use r2d2::Error as R2d2Error;
#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
 #[cfg(feature = "rocksdb-datastore")]
use bincode::Error as BincodeError;

use serde_json::Error as JsonError;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Json(JsonError);
        Postgres(PostgresError) #[cfg(feature = "postgres-datastore")];
        PostgresPool(R2d2Error) #[cfg(feature = "postgres-datastore")];
        RocksDb(RocksDbError) #[cfg(feature = "rocksdb-datastore")];
        Bincode(BincodeError) #[cfg(feature = "rocksdb-datastore")];
    }
}

error_chain! {
    types {
        ValidationError, ValidationErrorKind, ValidationResultExt, ValidationResult;
    }
}
