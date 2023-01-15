//! Scaffolding for testing and benchmarking. This exposes an implementation
//! of Datastore, so that the standard testing and benchmarking suite can be
//! reused. Under the hood, it uses a tokio runtime to call async functions
//! from non-async functions.

use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use std::time::Duration;

use indradb::QueryExt;

use tokio::runtime::Runtime;
use tokio::time::sleep;
use tonic::transport::Endpoint;
use uuid::Uuid;

fn map_client_result<T>(result: Result<T, crate::ClientError>) -> Result<T, indradb::Error> {
    result.map_err(|err| {
        match err {
            // this is the only error variant we need to handle for testing
            crate::ClientError::Grpc { inner }
                if inner.code() == tonic::Code::Internal
                    && inner.message() == "query attempted on a property that isn't indexed" =>
            {
                indradb::Error::NotIndexed
            }
            // unexpected error variant
            _ => panic!("{}", err),
        }
    })
}

pub struct ClientDatastore {
    client: Rc<RefCell<crate::Client>>,
    exec: Rc<RefCell<Runtime>>,
}

// the use of this function inside the full_test_impl macro appears to make
// the linter think this is dead code, so ignore the warning for now
#[allow(dead_code)]
impl ClientDatastore {
    pub fn new(port: u16, exec: Runtime) -> Self {
        let endpoint: Endpoint = format!("http://127.0.0.1:{}", port).try_into().unwrap();

        for _ in 0..5 {
            if let Ok(mut client) = exec.block_on(crate::Client::new(endpoint.clone())) {
                if exec.block_on(client.ping()).is_ok() {
                    return Self {
                        client: Rc::new(RefCell::new(client)),
                        exec: Rc::new(RefCell::new(exec)),
                    };
                }
            }

            exec.block_on(sleep(Duration::from_secs(1)));
        }

        panic!("Could not connect to the server after a few seconds");
    }

    fn get(&self, q: indradb::Query) -> indradb::Result<Vec<indradb::QueryOutputValue>> {
        map_client_result(self.exec.borrow_mut().block_on(self.client.borrow_mut().get(q)))
    }

    fn delete(&self, q: indradb::Query) -> indradb::Result<()> {
        map_client_result(self.exec.borrow_mut().block_on(self.client.borrow_mut().delete(q)))
    }

    fn set_properties(
        &self,
        q: indradb::Query,
        name: indradb::Identifier,
        value: serde_json::Value,
    ) -> indradb::Result<()> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().set_properties(q, name, value)),
        )
    }
}

impl indradb::tests::DatabaseV3 for ClientDatastore {
    fn get_vertices(&self, q: indradb::Query) -> indradb::Result<Vec<indradb::Vertex>> {
        indradb::util::extract_vertices(self.get(q)?).ok_or(indradb::Error::Unsupported)
    }

    fn delete_vertices(&self, q: indradb::Query) -> indradb::Result<()> {
        self.delete(q)
    }

    fn get_vertex_count(&self) -> indradb::Result<u64> {
        let q = indradb::AllVertexQuery.count().unwrap().into();
        indradb::util::extract_count(self.get(q)?).ok_or(indradb::Error::Unsupported)
    }

    fn get_edges(&self, q: indradb::Query) -> indradb::Result<Vec<indradb::Edge>> {
        indradb::util::extract_edges(self.get(q)?).ok_or(indradb::Error::Unsupported)
    }

    fn delete_edges(&self, q: indradb::Query) -> indradb::Result<()> {
        self.delete(q)
    }

    fn get_edge_count(
        &self,
        id: Uuid,
        t: Option<&indradb::Identifier>,
        direction: indradb::EdgeDirection,
    ) -> indradb::Result<u64> {
        let q = indradb::SpecificVertexQuery::single(id);

        let q = match direction {
            indradb::EdgeDirection::Outbound => q.outbound().unwrap(),
            indradb::EdgeDirection::Inbound => q.inbound().unwrap(),
        };

        let q: indradb::Query = if let Some(t) = t {
            q.t(t.clone()).count().unwrap().into()
        } else {
            q.count().unwrap().into()
        };

        indradb::util::extract_count(self.get(q)?).ok_or(indradb::Error::Unsupported)
    }

    fn get_vertex_properties(&self, q: indradb::PipePropertyQuery) -> indradb::Result<Vec<indradb::VertexProperty>> {
        let props = indradb::util::extract_vertex_properties(self.get(q.into())?).ok_or(indradb::Error::Unsupported)?;
        if props.len() > 1 {
            Err(indradb::Error::Unsupported)
        } else {
            let iter = props.into_iter().flat_map(|vps| {
                vps.props
                    .into_iter()
                    .map(move |vp| indradb::VertexProperty::new(vps.vertex.id, vp.value))
            });
            Ok(iter.collect())
        }
    }

    fn get_all_vertex_properties(&self, q: indradb::Query) -> indradb::Result<Vec<indradb::VertexProperties>> {
        let props_query = indradb::PipePropertyQuery::new(Box::new(q))?;
        let props = indradb::util::extract_vertex_properties(self.get(props_query.into())?)
            .ok_or(indradb::Error::Unsupported)?;
        Ok(props)
    }

    fn set_vertex_properties(&self, q: indradb::PipePropertyQuery, value: serde_json::Value) -> indradb::Result<()> {
        if let Some(name) = q.name {
            self.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(indradb::Error::Unsupported)
        }
    }

    fn delete_vertex_properties(&self, q: indradb::PipePropertyQuery) -> indradb::Result<()> {
        self.delete(q.into())
    }

    fn get_edge_properties(&self, q: indradb::PipePropertyQuery) -> indradb::Result<Vec<indradb::EdgeProperty>> {
        let props = indradb::util::extract_edge_properties(self.get(q.into())?).ok_or(indradb::Error::Unsupported)?;
        if props.len() > 1 {
            Err(indradb::Error::Unsupported)
        } else {
            let iter = props.into_iter().flat_map(move |eps| {
                let iter = eps
                    .props
                    .into_iter()
                    .map(|ep| indradb::EdgeProperty::new(eps.edge.clone(), ep.value));
                iter.collect::<Vec<indradb::EdgeProperty>>()
            });
            Ok(iter.collect())
        }
    }

    fn get_all_edge_properties(&self, q: indradb::Query) -> indradb::Result<Vec<indradb::EdgeProperties>> {
        let props_query = indradb::PipePropertyQuery::new(Box::new(q))?;
        indradb::util::extract_edge_properties(self.get(props_query.into())?).ok_or(indradb::Error::Unsupported)
    }

    fn set_edge_properties(&self, q: indradb::PipePropertyQuery, value: serde_json::Value) -> indradb::Result<()> {
        if let Some(name) = q.name {
            self.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(indradb::Error::Unsupported)
        }
    }

    fn delete_edge_properties(&self, q: indradb::PipePropertyQuery) -> indradb::Result<()> {
        self.delete(q.into())
    }

    fn create_vertex(&self, vertex: &indradb::Vertex) -> indradb::Result<bool> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().create_vertex(vertex)),
        )
    }

    fn create_edge(&self, edge: &indradb::Edge) -> indradb::Result<bool> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().create_edge(edge)),
        )
    }

    fn bulk_insert(&self, items: Vec<indradb::BulkInsertItem>) -> indradb::Result<()> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().bulk_insert(items)),
        )
    }

    fn index_property(&self, name: indradb::Identifier) -> indradb::Result<()> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().index_property(name)),
        )
    }
}

full_test_impl!({
    use std::net::ToSocketAddrs;
    use std::sync::Arc;
    use tokio::net::TcpListener;

    let rt = Runtime::new().unwrap();

    let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap();
    let listener = rt.block_on(TcpListener::bind(&addr)).unwrap();
    let port = listener.local_addr().unwrap().port();
    rt.spawn(crate::run_server(
        Arc::new(indradb::MemoryDatastore::new_db()),
        listener,
    ));

    ClientDatastore::new(port as u16, rt)
});
