use std::cmp::max;
use std::sync::{Arc, Mutex};

use crate::errors::Error;

use indradb::util::extract_vertices;
use threadpool::ThreadPool;

const DEFAULT_NUM_THREADS: usize = 8;
const DEFAULT_QUERY_LIMIT: u32 = u16::max_value() as u32;

/// Trait for running an operation on all vertices in a datastore.
pub trait VertexMapper: Send + Sync + 'static {
    /// The number of threads that should execute the map operation.
    fn num_threads(&self) -> usize {
        DEFAULT_NUM_THREADS
    }
    /// How many vertices to pull at a time.
    fn query_limit(&self) -> u32 {
        DEFAULT_QUERY_LIMIT
    }
    /// If specified, only vertices of the specified type will be mapped.
    fn t_filter(&self) -> Option<indradb::Identifier> {
        None
    }
    /// The map operation.
    fn map(&self, vertex: indradb::Vertex) -> Result<(), Error>;
}

/// Runs an operation on all vertices in the datastore.
///
/// # Arguments
/// * `mapper`: Specified options and the map operation to run.
/// * `database`: The database.
pub fn map<M: VertexMapper, D: indradb::Datastore + Send + Sync + 'static>(
    mapper: Arc<M>,
    db: &indradb::Database<D>,
) -> Result<(), Error> {
    let pool = ThreadPool::new(max(mapper.num_threads(), 1));
    let query_limit = max(mapper.query_limit(), 1);
    let t_filter = mapper.t_filter();
    let last_err: Arc<Mutex<Option<Error>>> = Arc::new(Mutex::new(None));
    let mut last_id: Option<uuid::Uuid> = None;

    loop {
        if last_err.lock().unwrap().is_some() {
            break;
        }

        let mut q = indradb::RangeVertexQuery::new().limit(query_limit);
        if let Some(ref t_filter) = t_filter {
            q = q.t(t_filter.clone());
        }
        if let Some(last_id) = last_id {
            q = q.start_id(last_id);
        }

        let output = match db.get(q.into()) {
            Ok(output) => output,
            Err(err) => {
                *last_err.lock().unwrap() = Some(err.into());
                break;
            }
        };

        let vertices = extract_vertices(output).unwrap();

        let is_last_query = vertices.len() < query_limit as usize;
        if let Some(last_vertex) = vertices.last() {
            last_id = Some(last_vertex.id);
        }

        for vertex in vertices {
            let mapper = mapper.clone();
            let last_err = last_err.clone();
            pool.execute(move || {
                if let Err(err) = mapper.map(vertex) {
                    *last_err.lock().unwrap() = Some(err);
                }
            });
        }

        if is_last_query {
            break;
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
