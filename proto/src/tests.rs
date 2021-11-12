//! Scaffolding for testing and benchmarking. This exposes implementations of
//! Datastore and Transaction, so that the standard testing and benchmarking
//! suite can be reused. Under the hood, they use tokio runtimes to call async
//! functions from non-async functions.

use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use std::time::Duration;

use serde_json::value::Value as JsonValue;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use tonic::transport::Endpoint;
use uuid::Uuid;

struct ClientDatastore {
    client: Rc<RefCell<crate::Client>>,
    exec: Rc<RefCell<Runtime>>,
}

// the use of this function inside the full_test_impl macro appears to make
// the linter think this is dead code, so ignore the warning for now
#[allow(dead_code)]
impl ClientDatastore {
    fn new(port: u16, exec: Runtime) -> Self {
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
}

impl indradb::Datastore for ClientDatastore {
    type Trans = ClientTransaction;

    fn sync(&self) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .block_on(self.client.borrow_mut().sync())
            .unwrap();
        Ok(())
    }

    fn bulk_insert<I>(&self, items: I) -> Result<(), indradb::Error>
    where
        I: Iterator<Item = indradb::BulkInsertItem>,
    {
        self.exec
            .borrow_mut()
            .block_on(self.client.borrow_mut().bulk_insert(items))
            .unwrap();
        Ok(())
    }

    fn transaction(&self) -> Result<ClientTransaction, indradb::Error> {
        let trans = self
            .exec
            .borrow_mut()
            .block_on(self.client.borrow_mut().transaction())
            .unwrap();
        Ok(ClientTransaction::new(trans, self.exec.clone()))
    }

    fn index_property<T: Into<indradb::Type>>(&mut self, name: T) -> Result<()> {
        self.exec
            .borrow_mut()
            .block_on(self.client.borrow_mut().index_property(name))
            .unwrap();
        Ok(())
    }
}

struct ClientTransaction {
    trans: Rc<RefCell<crate::Transaction>>,
    exec: Rc<RefCell<Runtime>>,
}

impl ClientTransaction {
    fn new(trans: crate::Transaction, exec: Rc<RefCell<Runtime>>) -> Self {
        ClientTransaction {
            trans: Rc::new(RefCell::new(trans)),
            exec,
        }
    }
}

impl indradb::Transaction for ClientTransaction {
    fn create_vertex(&self, v: &indradb::Vertex) -> Result<bool, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().create_vertex(v))
            .unwrap())
    }

    fn create_vertex_from_type(&self, t: indradb::Type) -> Result<Uuid, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().create_vertex_from_type(t))
            .unwrap())
    }

    fn get_vertices<Q: Into<indradb::VertexQuery>>(&self, q: Q) -> Result<Vec<indradb::Vertex>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_vertices(q))
            .unwrap())
    }

    fn delete_vertices<Q: Into<indradb::VertexQuery>>(&self, q: Q) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().delete_vertices(q))
            .unwrap();
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_vertex_count())
            .unwrap())
    }

    fn create_edge(&self, e: &indradb::EdgeKey) -> Result<bool, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().create_edge(e))
            .unwrap())
    }

    fn get_edges<Q: Into<indradb::EdgeQuery>>(&self, q: Q) -> Result<Vec<indradb::Edge>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_edges(q))
            .unwrap())
    }

    fn delete_edges<Q: Into<indradb::EdgeQuery>>(&self, q: Q) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().delete_edges(q))
            .unwrap();
        Ok(())
    }

    fn get_edge_count(
        &self,
        id: Uuid,
        t: Option<&indradb::Type>,
        direction: indradb::EdgeDirection,
    ) -> Result<u64, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_edge_count(id, t, direction))
            .unwrap())
    }

    fn get_vertex_properties(
        &self,
        q: indradb::VertexPropertyQuery,
    ) -> Result<Vec<indradb::VertexProperty>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_vertex_properties(q))
            .unwrap())
    }

    fn get_all_vertex_properties<Q: Into<indradb::VertexQuery>>(
        &self,
        q: Q,
    ) -> Result<Vec<indradb::VertexProperties>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_all_vertex_properties(q))
            .unwrap())
    }

    fn set_vertex_properties(&self, q: indradb::VertexPropertyQuery, value: &JsonValue) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().set_vertex_properties(q, value))
            .unwrap();
        Ok(())
    }

    fn delete_vertex_properties(&self, q: indradb::VertexPropertyQuery) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().delete_vertex_properties(q))
            .unwrap();
        Ok(())
    }

    fn get_edge_properties(&self, q: indradb::EdgePropertyQuery) -> Result<Vec<indradb::EdgeProperty>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_edge_properties(q))
            .unwrap())
    }

    fn get_all_edge_properties<Q: Into<indradb::EdgeQuery>>(
        &self,
        q: Q,
    ) -> Result<Vec<indradb::EdgeProperties>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().get_all_edge_properties(q))
            .unwrap())
    }

    fn set_edge_properties(&self, q: indradb::EdgePropertyQuery, value: &JsonValue) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().set_edge_properties(q, value))
            .unwrap();
        Ok(())
    }

    fn delete_edge_properties(&self, q: indradb::EdgePropertyQuery) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .block_on(self.trans.borrow_mut().delete_edge_properties(q))
            .unwrap();
        Ok(())
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
        Arc::new(indradb::MemoryDatastore::default()),
        listener,
    ));

    ClientDatastore::new(port as u16, rt)
});
