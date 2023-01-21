//! Scaffolding for testing and benchmarking. This exposes an implementation
//! of `Datastore` and `Transaction`. Under the hood, it uses a tokio runtime
//! to call async functions from non-async functions. This allows us to reuse
//! the standard testing suite, but it's a huge hack! It's trait
//! implementations that are simulating low-level functionality (stuff that
//! runs below the database) via the high-level client (stuff that runs above
//! the database.)

use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use std::result::Result as StdResult;
use std::time::Duration;

use indradb::{
    util, AllEdgeQuery, AllVertexQuery, BulkInsertItem, Datastore, DynIter, Edge, EdgeWithPropertyPresenceQuery,
    EdgeWithPropertyValueQuery, Error, Identifier, Query, QueryExt, QueryOutputValue, RangeVertexQuery, Result,
    SpecificEdgeQuery, SpecificVertexQuery, Transaction, Vertex, VertexWithPropertyPresenceQuery,
    VertexWithPropertyValueQuery,
};

use tokio::runtime::Runtime;
use tokio::time::sleep;
use tonic::transport::Endpoint;
use uuid::Uuid;

fn map_client_result<T>(result: StdResult<T, crate::ClientError>) -> Result<T> {
    result.map_err(|err| {
        match err {
            // this is the only error variant we need to handle for testing
            crate::ClientError::Grpc { inner }
                if inner.code() == tonic::Code::Internal
                    && inner.message() == "query attempted on a property that isn't indexed" =>
            {
                Error::NotIndexed
            }
            // unexpected error variant
            _ => panic!("{}", err),
        }
    })
}

pub struct ClientTransaction {
    client: Rc<RefCell<crate::Client>>,
    exec: Rc<RefCell<Runtime>>,
}

impl<'a> ClientTransaction {
    fn get<Q: Into<Query>>(&self, q: Q) -> Result<Vec<QueryOutputValue>> {
        map_client_result(self.exec.borrow_mut().block_on(self.client.borrow_mut().get(q)))
    }

    fn delete<Q: Into<Query>>(&self, q: Q) -> Result<()> {
        map_client_result(self.exec.borrow_mut().block_on(self.client.borrow_mut().delete(q)))
    }

    fn set_properties<Q: Into<Query>>(&self, q: Q, name: Identifier, value: serde_json::Value) -> Result<()> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().set_properties(q, name, value)),
        )
    }

    fn get_count<Q: Into<Query>>(&self, q: Q) -> u64 {
        util::extract_count(self.get(q).unwrap()).unwrap()
    }

    fn get_vertices<Q: Into<Query>>(&'a self, q: Q) -> Result<DynIter<'a, Vertex>> {
        let vertices = util::extract_vertices(self.get(q)?).ok_or(Error::Unsupported)?;
        Ok(Box::new(vertices.into_iter().map(Ok)))
    }

    fn get_edges<Q: Into<Query>>(&'a self, q: Q) -> Result<DynIter<'a, Edge>> {
        let edges = util::extract_edges(self.get(q)?).ok_or(Error::Unsupported)?;
        Ok(Box::new(edges.into_iter().map(Ok)))
    }
}

impl<'a> Transaction<'a> for ClientTransaction {
    fn vertex_count(&self) -> u64 {
        self.get_count(AllVertexQuery.count().unwrap())
    }

    fn all_vertices(&'a self) -> Result<DynIter<'a, Vertex>> {
        self.get_vertices(AllVertexQuery)
    }

    fn range_vertices(&'a self, offset: Uuid) -> Result<DynIter<'a, Vertex>> {
        self.get_vertices(RangeVertexQuery::default().start_id(offset))
    }

    fn specific_vertices(&'a self, ids: Vec<Uuid>) -> Result<DynIter<'a, Vertex>> {
        self.get_vertices(SpecificVertexQuery::new(ids))
    }

    fn vertex_ids_with_property(&'a self, name: &Identifier) -> Result<Option<DynIter<'a, Uuid>>> {
        let q = VertexWithPropertyPresenceQuery::new(name.clone());
        let vertices = util::extract_vertices(self.get(q)?).unwrap();
        Ok(Some(Box::new(vertices.into_iter().map(|v| Ok(v.id)))))
    }

    fn vertex_ids_with_property_value(
        &'a self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Uuid>>> {
        let q = VertexWithPropertyValueQuery::new(name.clone(), value.clone());
        let vertices = util::extract_vertices(self.get(q)?).unwrap();
        Ok(Some(Box::new(vertices.into_iter().map(|v| Ok(v.id)))))
    }

    fn edge_count(&self) -> u64 {
        self.get_count(AllEdgeQuery.count().unwrap())
    }

    fn all_edges(&'a self) -> Result<DynIter<'a, Edge>> {
        self.get_edges(AllEdgeQuery)
    }

    fn range_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        let edges = util::extract_edges(self.get(AllEdgeQuery)?).unwrap();
        let iter = edges.into_iter().filter(move |e| e >= &offset).map(Ok);
        Ok(Box::new(iter))
    }

    fn range_reversed_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        let edges = util::extract_edges(self.get(AllEdgeQuery)?).unwrap();
        let iter = edges
            .into_iter()
            .map(|e| e.reversed())
            .filter(move |e| e >= &offset)
            .map(Ok);
        Ok(Box::new(iter))
    }

    fn specific_edges(&'a self, edges: Vec<Edge>) -> Result<DynIter<'a, Edge>> {
        self.get_edges(SpecificEdgeQuery::new(edges))
    }

    fn edges_with_property(&'a self, name: &Identifier) -> Result<Option<DynIter<'a, Edge>>> {
        let q = EdgeWithPropertyPresenceQuery::new(name.clone());
        let edges = util::extract_edges(self.get(q)?).unwrap();
        Ok(Some(Box::new(edges.into_iter().map(Ok))))
    }

    fn edges_with_property_value(
        &'a self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Edge>>> {
        let q = EdgeWithPropertyValueQuery::new(name.clone(), value.clone());
        let edges = util::extract_edges(self.get(q)?).unwrap();
        Ok(Some(Box::new(edges.into_iter().map(Ok))))
    }

    fn vertex_property(&self, vertex: &Vertex, name: &Identifier) -> Result<Option<serde_json::Value>> {
        let q = SpecificVertexQuery::single(vertex.id)
            .properties()
            .unwrap()
            .name(name.clone());
        let props = util::extract_vertex_properties(self.get(q)?).unwrap();
        match props.len() {
            0 => Ok(None),
            1 => match props[0].props.len() {
                0 => Ok(None),
                1 => Ok(Some(props[0].props[0].value.clone())),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn all_vertex_properties_for_vertex(
        &'a self,
        vertex: &Vertex,
    ) -> Result<DynIter<'a, (Identifier, serde_json::Value)>> {
        let q = SpecificVertexQuery::single(vertex.id).properties().unwrap();
        let props = util::extract_vertex_properties(self.get(q)?).unwrap();
        match props.len() {
            0 => Ok(Box::new(Vec::default().into_iter())),
            1 => {
                let props: Vec<Result<(Identifier, serde_json::Value)>> = props[0]
                    .props
                    .iter()
                    .map(|p| Ok((p.name.clone(), p.value.clone())))
                    .collect();
                Ok(Box::new(props.into_iter()))
            }
            _ => unreachable!(),
        }
    }

    fn edge_property(&self, edge: &Edge, name: &Identifier) -> Result<Option<serde_json::Value>> {
        let q = SpecificEdgeQuery::single(edge.clone())
            .properties()
            .unwrap()
            .name(name.clone());
        let props = util::extract_edge_properties(self.get(q)?).unwrap();
        match props.len() {
            0 => Ok(None),
            1 => match props[0].props.len() {
                0 => Ok(None),
                1 => Ok(Some(props[0].props[0].value.clone())),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn all_edge_properties_for_edge(&'a self, edge: &Edge) -> Result<DynIter<'a, (Identifier, serde_json::Value)>> {
        let q = SpecificEdgeQuery::single(edge.clone()).properties().unwrap();
        let props = util::extract_edge_properties(self.get(q)?).unwrap();
        match props.len() {
            0 => Ok(Box::new(Vec::default().into_iter())),
            1 => {
                let props: Vec<Result<(Identifier, serde_json::Value)>> = props[0]
                    .props
                    .iter()
                    .map(|p| Ok((p.name.clone(), p.value.clone())))
                    .collect();
                Ok(Box::new(props.into_iter()))
            }
            _ => unreachable!(),
        }
    }

    fn delete_vertices(&mut self, vertices: Vec<Vertex>) -> Result<()> {
        self.delete(SpecificVertexQuery::new(vertices.into_iter().map(|v| v.id).collect()))
    }

    fn delete_edges(&mut self, edges: Vec<Edge>) -> Result<()> {
        self.delete(SpecificEdgeQuery::new(edges))
    }

    fn delete_vertex_properties(&mut self, props: Vec<(Uuid, Identifier)>) -> Result<()> {
        for (id, name) in props {
            self.delete(SpecificVertexQuery::single(id).properties().unwrap().name(name))?;
        }
        Ok(())
    }

    fn delete_edge_properties(&mut self, props: Vec<(Edge, Identifier)>) -> Result<()> {
        for (edge, name) in props {
            self.delete(SpecificEdgeQuery::single(edge).properties().unwrap().name(name))?;
        }
        Ok(())
    }

    fn create_vertex(&mut self, vertex: &Vertex) -> Result<bool> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().create_vertex(vertex)),
        )
    }

    fn create_edge(&mut self, edge: &Edge) -> Result<bool> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().create_edge(edge)),
        )
    }

    fn bulk_insert(&mut self, items: Vec<BulkInsertItem>) -> Result<()> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().bulk_insert(items)),
        )
    }

    fn index_property(&mut self, name: Identifier) -> Result<()> {
        map_client_result(
            self.exec
                .borrow_mut()
                .block_on(self.client.borrow_mut().index_property(name)),
        )
    }

    fn set_vertex_properties(
        &mut self,
        vertex_ids: Vec<Uuid>,
        name: Identifier,
        value: serde_json::Value,
    ) -> Result<()> {
        self.set_properties(SpecificVertexQuery::new(vertex_ids), name, value)
    }

    fn set_edge_properties(&mut self, edges: Vec<Edge>, name: Identifier, value: serde_json::Value) -> Result<()> {
        self.set_properties(SpecificEdgeQuery::new(edges), name, value)
    }
}

pub struct ClientDatastore {
    client: Rc<RefCell<crate::Client>>,
    exec: Rc<RefCell<Runtime>>,
}

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
}

impl Datastore for ClientDatastore {
    type Transaction<'a> = ClientTransaction;
    fn transaction(&'_ self) -> Self::Transaction<'_> {
        ClientTransaction {
            client: self.client.clone(),
            exec: self.exec.clone(),
        }
    }
}

full_test_impl!({
    use indradb::Database;
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

    Database::new(ClientDatastore::new(port as u16, rt))
});
