use std::cmp::max;
use std::sync::{Arc, Mutex};

use crate::errors::Error;

use threadpool::ThreadPool;

const DEFAULT_NUM_WORKERS: usize = 8;
const DEFAULT_QUERY_LIMIT: u32 = u16::max_value() as u32;

pub trait VertexMapper: Send + Sync + 'static {
    fn num_workers(&self) -> usize {
        DEFAULT_NUM_WORKERS
    }
    fn query_limit(&self) -> u32 {
        DEFAULT_QUERY_LIMIT
    }
    fn t_filter(&self) -> Option<indradb::Identifier> {
        None
    }
    fn map(&self, vertex: indradb::Vertex) -> Result<(), Error>;
}

pub fn map<M: VertexMapper>(
    mapper: Arc<M>,
    datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
) -> Result<(), Error> {
    let pool = ThreadPool::new(max(mapper.num_workers(), 2));
    let query_limit = max(mapper.query_limit(), 1);
    let t_filter = mapper.t_filter();
    let last_err: Arc<Mutex<Option<Error>>> = Arc::new(Mutex::new(None));
    let mut last_id: Option<uuid::Uuid> = None;

    loop {
        if last_err.lock().unwrap().is_some() {
            break;
        }

        let q = indradb::RangeVertexQuery {
            limit: query_limit,
            t: t_filter.clone(),
            start_id: last_id,
        };

        let vertices = match datastore.get_vertices(q.into()) {
            Ok(value) => value,
            Err(err) => {
                *last_err.lock().unwrap() = Some(err.into());
                break;
            }
        };

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
