use std::cmp::max;
use std::sync::{Arc, Mutex};

use crate::errors::Error;

use threadpool::ThreadPool;

const DEFAULT_NUM_THREADS: usize = 8;

/// Trait for running an operation on all vertices in a datastore.
pub trait VertexMapper: Send + Sync + 'static {
    /// The number of threads that should execute the map operation.
    fn num_threads(&self) -> usize {
        DEFAULT_NUM_THREADS
    }
    /// The map operation.
    fn map(&self, vertex: indradb::Vertex) -> Result<(), Error>;
}

/// Runs an operation on all vertices in the datastore.
///
/// # Arguments
/// * `mapper`: Specified options and the map operation to run.
/// * `database`: The database.
pub fn map<'a, M: VertexMapper>(txn: &'a dyn indradb::Transaction<'a>, mapper: Arc<M>) -> Result<(), Error> {
    let last_err: Arc<Mutex<Option<Error>>> = Arc::new(Mutex::new(None));
    let pool = ThreadPool::new(max(mapper.num_threads(), 1));
    let txn_ptr = txn as *const dyn indradb::Transaction<'a>;

    for vertex in unsafe { (*txn_ptr).all_vertices()? } {
        if last_err.lock().unwrap().is_some() {
            break;
        }
        match vertex {
            Ok(vertex) => {
                let vertex: indradb::Vertex = vertex;
                let mapper = mapper.clone();
                let last_err = last_err.clone();
                pool.execute(move || {
                    if let Err(err) = mapper.map(vertex) {
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
