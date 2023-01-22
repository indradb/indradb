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
pub fn map<'a, M: VertexMapper>(txn: &(dyn indradb::Transaction<'a> + 'a), mapper: Arc<M>) -> Result<(), Error> {
    let first_err: Arc<Mutex<Option<Error>>> = Arc::new(Mutex::new(None));
    let pool = ThreadPool::new(max(mapper.num_threads(), 1));
    let txn_ptr = txn as *const dyn indradb::Transaction<'a>;
    let mut i = 0;

    for vertex in unsafe { (*txn_ptr).all_vertices()? } {
        i += 1;
        if i % 1000 == 0 && first_err.lock().unwrap().is_some() {
            // Break on error, but also only check every once in a while since
            // the error is behind a mutex.
            break;
        }
        match vertex {
            Ok(vertex) => {
                let vertex: indradb::Vertex = vertex;
                let mapper = mapper.clone();
                let first_err = first_err.clone();
                pool.execute(move || {
                    if let Err(err) = mapper.map(vertex) {
                        let mut first_err = first_err.lock().unwrap();
                        if first_err.is_none() {
                            *first_err = Some(err);
                        }
                    }
                });
            }
            Err(err) => {
                let mut first_err = first_err.lock().unwrap();
                if first_err.is_none() {
                    *first_err = Some(Error::IndraDB(err));
                }
            }
        }
    }

    pool.join();

    let mut first_err = first_err.lock().unwrap();
    if first_err.is_some() {
        Err(first_err.take().unwrap())
    } else {
        Ok(())
    }
}
