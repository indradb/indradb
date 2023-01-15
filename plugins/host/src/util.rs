use std::cmp::max;
use std::sync::{Arc, Mutex};

use crate::errors::Error;
use crate::DynTransaction;

use threadpool::ThreadPool;

const DEFAULT_NUM_THREADS: usize = 8;

/// Runs an operation on all vertices in the datastore.
///
/// # Arguments
/// * `mapper`: Specified options and the map operation to run.
/// * `database`: The database.
pub fn map<'a, F: FnOnce(indradb::Vertex) -> Result<(), Error> + Send + Clone>(
    txn: DynTransaction<'a>,
    mapper: F,
    num_threads: Option<usize>,
) -> Result<(), Error> {
    let pool = ThreadPool::new(max(num_threads.unwrap_or(DEFAULT_NUM_THREADS), 1));
    let last_err: Arc<Mutex<Option<Error>>> = Arc::new(Mutex::new(None));

    for vertex in txn.all_vertices()? {
        if last_err.lock().unwrap().is_some() {
            break;
        }
        match vertex {
            Ok(vertex) => {
                let vertex: indradb::Vertex = vertex;
                let mapper = mapper.clone();
                let last_err = last_err.clone();
                pool.execute(move || {
                    if let Err(err) = mapper(vertex) {
                        *last_err.lock().unwrap() = Some(err);
                    }
                });
            }
            Err(err) => {
                *last_err.lock().unwrap() = Some(Error::IndraDB(err));
                break;
            }
        }
    }

    pool.join();

    let mut last_err = last_err.lock().unwrap();
    if last_err.is_some() {
        Err(last_err.take().unwrap())
    } else {
        Ok(())
    }
}
