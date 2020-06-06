use std::cell::RefCell;
use std::net::ToSocketAddrs;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use crate::autogen;
use crate::converters;

use capnp::Error as CapnpError;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::{twoparty, RpcSystem};
use futures::executor::{LocalPool, LocalSpawner};
use futures::prelude::*;
use futures::task::LocalSpawn;
use indradb;
use serde_json::value::Value as JsonValue;

async fn build_client(port: u16, spawner: &LocalSpawner) -> Result<autogen::service::Client, CapnpError> {
    let addr = format!("127.0.0.1:{}", port).to_socket_addrs().unwrap().next().unwrap();
    let stream = async_std::net::TcpStream::connect(&addr).await?;
    stream.set_nodelay(true)?;
    let (reader, writer) = stream.split();

    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        Side::Client,
        Default::default(),
    ));
    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let client: autogen::service::Client = rpc_system.bootstrap(Side::Server);

    spawner
        .spawn_local_obj(Box::pin(rpc_system.map(|_| ())).into())
        .map_err(|err| CapnpError::failed(format!("spawn failed: {}", err)))?;

    let req = client.ping_request();
    let res = req.send().promise.await?;

    if res.get().unwrap().get_ready() {
        Ok(client)
    } else {
        Err(CapnpError::failed("not ready yet".to_string()))
    }
}

pub struct ClientDatastore {
    client: autogen::service::Client,
    exec: Rc<RefCell<LocalPool>>,
}

impl ClientDatastore {
    pub fn new(port: u16, mut exec: LocalPool) -> Self {
        let spawner = exec.spawner();

        for _ in 0..10 {
            if let Ok(client) = exec.run_until(build_client(port, &spawner)) {
                return Self {
                    client,
                    exec: Rc::new(RefCell::new(exec)),
                };
            }

            sleep(Duration::from_secs(1));
        }

        panic!("Could not connect to the server after a few seconds");
    }
}

impl ClientDatastore {
    async fn async_bulk_insert<I>(&self, items: I) -> Result<indradb::BulkInsertResult, CapnpError>
    where
        I: Iterator<Item = indradb::BulkInsertItem>,
    {
        let items: Vec<indradb::BulkInsertItem> = items.collect();
        let mut req = self.client.bulk_insert_request();
        converters::from_bulk_insert_items(&items, req.get().init_items(items.len() as u32)).unwrap();
        let res = req.send().promise.await?;
        let reader = res.get()?.get_result()?;
        Ok(indradb::BulkInsertResult {
            id_range: if reader.get_start_id() == 0 {
                None
            } else {
                Some((reader.get_start_id(), reader.get_end_id()))
            }
        })
    }
}

impl indradb::Datastore for ClientDatastore {
    type Trans = ClientTransaction;

    fn bulk_insert<I>(&self, items: I) -> Result<indradb::BulkInsertResult, indradb::Error>
    where
        I: Iterator<Item = indradb::BulkInsertItem>,
    {
        Ok(self
            .exec
            .borrow_mut()
            .run_until(self.async_bulk_insert(items))
            .unwrap())
    }

    fn transaction(&self) -> Result<ClientTransaction, indradb::Error> {
        let trans = self.client.transaction_request().send().pipeline.get_transaction();
        Ok(ClientTransaction::new(trans, self.exec.clone()))
    }
}

pub struct ClientTransaction {
    trans: RefCell<autogen::transaction::Client>,
    exec: Rc<RefCell<LocalPool>>,
}

impl ClientTransaction {
    fn new(trans: autogen::transaction::Client, exec: Rc<RefCell<LocalPool>>) -> Self {
        ClientTransaction {
            trans: RefCell::new(trans),
            exec,
        }
    }
}

impl ClientTransaction {
    async fn async_create_vertex(&self, t: &indradb::Type) -> Result<u64, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.create_vertex_request();
        req.get().set_t(&t.0);
        let res = req.send().promise.await?;
        let id = res.get()?.get_result();
        Ok(id)
    }

    async fn async_get_vertices<Q: Into<indradb::VertexQuery>>(
        &self,
        q: Q,
    ) -> Result<Vec<indradb::Vertex>, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.get_vertices_request();
        converters::from_vertex_query(&q.into(), req.get().init_q());
        let res = req.send().promise.await?;
        let list = res.get()?.get_result()?;
        let list: Result<Vec<indradb::Vertex>, CapnpError> =
            list.into_iter().map(|reader| converters::to_vertex(&reader)).collect();
        list
    }

    async fn async_delete_vertices<Q: Into<indradb::VertexQuery>>(&self, q: Q) -> Result<(), CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.delete_vertices_request();
        converters::from_vertex_query(&q.into(), req.get().init_q());
        let res = req.send().promise.await?;
        res.get()?;
        Ok(())
    }

    async fn async_get_vertex_count(&self) -> Result<u64, CapnpError> {
        let trans = self.trans.borrow_mut();
        let req = trans.get_vertex_count_request();
        let res = req.send().promise.await?;
        Ok(res.get()?.get_result())
    }

    async fn async_create_edge(&self, e: &indradb::Edge) -> Result<bool, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.create_edge_request();
        converters::from_edge(e, req.get().init_edge());
        let res = req.send().promise.await?;
        Ok(res.get()?.get_result())
    }

    async fn async_get_edges<Q: Into<indradb::EdgeQuery>>(&self, q: Q) -> Result<Vec<indradb::Edge>, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.get_edges_request();
        converters::from_edge_query(&q.into(), req.get().init_q());

        let res = req.send().promise.await?;
        let list = res.get()?.get_result()?;
        let list: Result<Vec<indradb::Edge>, CapnpError> =
            list.into_iter().map(|reader| converters::to_edge(&reader)).collect();
        list
    }

    async fn async_delete_edges<Q: Into<indradb::EdgeQuery>>(&self, q: Q) -> Result<(), CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.delete_edges_request();
        converters::from_edge_query(&q.into(), req.get().init_q());
        let res = req.send().promise.await?;
        res.get()?;
        Ok(())
    }

    async fn async_get_edge_count(
        &self,
        id: u64,
        t: Option<&indradb::Type>,
        direction: indradb::EdgeDirection,
    ) -> Result<u64, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.get_edge_count_request();
        req.get().set_id(id);

        if let Some(t) = t {
            req.get().set_t(&t.0);
        }

        req.get().set_direction(converters::from_edge_direction(direction));

        let res = req.send().promise.await?;
        Ok(res.get()?.get_result())
    }

    async fn async_get_vertex_properties(
        &self,
        q: indradb::VertexPropertyQuery,
    ) -> Result<Vec<indradb::VertexProperty>, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.get_vertex_properties_request();
        converters::from_vertex_property_query(&q, req.get().init_q());

        let res = req.send().promise.await?;
        let list = res.get()?.get_result()?;
        let list: Result<Vec<indradb::VertexProperty>, CapnpError> = list
            .into_iter()
            .map(|reader| converters::to_vertex_property(&reader))
            .collect();
        list
    }

    async fn async_get_all_vertex_properties<Q: Into<indradb::VertexQuery>>(
        &self,
        q: Q,
    ) -> Result<Vec<indradb::VertexProperties>, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.get_all_vertex_properties_request();
        converters::from_vertex_query(&q.into(), req.get().init_q());

        let res = req.send().promise.await?;
        let list = res.get()?.get_result()?;
        let list: Result<Vec<indradb::VertexProperties>, CapnpError> = list
            .into_iter()
            .map(|reader| converters::to_vertex_properties(&reader))
            .collect();
        list
    }

    async fn async_set_vertex_properties(
        &self,
        q: indradb::VertexPropertyQuery,
        value: &JsonValue,
    ) -> Result<(), CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.set_vertex_properties_request();
        converters::from_vertex_property_query(&q, req.get().init_q());
        req.get().set_value(&value.to_string());

        let res = req.send().promise.await?;
        res.get()?;
        Ok(())
    }

    async fn async_delete_vertex_properties(&self, q: indradb::VertexPropertyQuery) -> Result<(), CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.delete_vertex_properties_request();
        converters::from_vertex_property_query(&q, req.get().init_q());

        let res = req.send().promise.await?;
        res.get()?;
        Ok(())
    }

    async fn async_get_edge_properties(
        &self,
        q: indradb::EdgePropertyQuery,
    ) -> Result<Vec<indradb::EdgeProperty>, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.get_edge_properties_request();
        converters::from_edge_property_query(&q, req.get().init_q());

        let res = req.send().promise.await?;
        let list = res.get()?.get_result()?;
        let list: Result<Vec<indradb::EdgeProperty>, CapnpError> = list
            .into_iter()
            .map(|reader| converters::to_edge_property(&reader))
            .collect();
        list
    }

    async fn async_get_all_edge_properties<Q: Into<indradb::EdgeQuery>>(
        &self,
        q: Q,
    ) -> Result<Vec<indradb::EdgeProperties>, CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.get_all_edge_properties_request();
        converters::from_edge_query(&q.into(), req.get().init_q());

        let res = req.send().promise.await?;
        let list = res.get()?.get_result()?;
        let list: Result<Vec<indradb::EdgeProperties>, CapnpError> = list
            .into_iter()
            .map(|reader| converters::to_edge_properties(&reader))
            .collect();
        list
    }

    async fn async_set_edge_properties(
        &self,
        q: indradb::EdgePropertyQuery,
        value: &JsonValue,
    ) -> Result<(), CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.set_edge_properties_request();
        converters::from_edge_property_query(&q, req.get().init_q());
        req.get().set_value(&value.to_string());

        let res = req.send().promise.await?;
        res.get()?;
        Ok(())
    }

    async fn async_delete_edge_properties(&self, q: indradb::EdgePropertyQuery) -> Result<(), CapnpError> {
        let trans = self.trans.borrow_mut();
        let mut req = trans.delete_edge_properties_request();
        converters::from_edge_property_query(&q, req.get().init_q());

        let res = req.send().promise.await?;
        res.get()?;
        Ok(())
    }
}

impl indradb::Transaction for ClientTransaction {
    fn create_vertex(&self, t: &indradb::Type) -> Result<u64, indradb::Error> {
        Ok(self.exec.borrow_mut().run_until(self.async_create_vertex(t)).unwrap())
    }

    fn get_vertices<Q: Into<indradb::VertexQuery>>(&self, q: Q) -> Result<Vec<indradb::Vertex>, indradb::Error> {
        Ok(self.exec.borrow_mut().run_until(self.async_get_vertices(q)).unwrap())
    }

    fn delete_vertices<Q: Into<indradb::VertexQuery>>(&self, q: Q) -> Result<(), indradb::Error> {
        self.exec.borrow_mut().run_until(self.async_delete_vertices(q)).unwrap();
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64, indradb::Error> {
        Ok(self.exec.borrow_mut().run_until(self.async_get_vertex_count()).unwrap())
    }

    fn create_edge(&self, e: &indradb::Edge) -> Result<bool, indradb::Error> {
        Ok(self.exec.borrow_mut().run_until(self.async_create_edge(e)).unwrap())
    }

    fn get_edges<Q: Into<indradb::EdgeQuery>>(&self, q: Q) -> Result<Vec<indradb::Edge>, indradb::Error> {
        Ok(self.exec.borrow_mut().run_until(self.async_get_edges(q)).unwrap())
    }

    fn delete_edges<Q: Into<indradb::EdgeQuery>>(&self, q: Q) -> Result<(), indradb::Error> {
        self.exec.borrow_mut().run_until(self.async_delete_edges(q)).unwrap();
        Ok(())
    }

    fn get_edge_count(
        &self,
        id: u64,
        t: Option<&indradb::Type>,
        direction: indradb::EdgeDirection,
    ) -> Result<u64, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .run_until(self.async_get_edge_count(id, t, direction))
            .unwrap())
    }

    fn get_vertex_properties(
        &self,
        q: indradb::VertexPropertyQuery,
    ) -> Result<Vec<indradb::VertexProperty>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .run_until(self.async_get_vertex_properties(q))
            .unwrap())
    }

    fn get_all_vertex_properties<Q: Into<indradb::VertexQuery>>(
        &self,
        q: Q,
    ) -> Result<Vec<indradb::VertexProperties>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .run_until(self.async_get_all_vertex_properties(q))
            .unwrap())
    }

    fn set_vertex_properties(&self, q: indradb::VertexPropertyQuery, value: &JsonValue) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .run_until(self.async_set_vertex_properties(q, value))
            .unwrap();
        Ok(())
    }

    fn delete_vertex_properties(&self, q: indradb::VertexPropertyQuery) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .run_until(self.async_delete_vertex_properties(q))
            .unwrap();
        Ok(())
    }

    fn get_edge_properties(&self, q: indradb::EdgePropertyQuery) -> Result<Vec<indradb::EdgeProperty>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .run_until(self.async_get_edge_properties(q))
            .unwrap())
    }

    fn get_all_edge_properties<Q: Into<indradb::EdgeQuery>>(
        &self,
        q: Q,
    ) -> Result<Vec<indradb::EdgeProperties>, indradb::Error> {
        Ok(self
            .exec
            .borrow_mut()
            .run_until(self.async_get_all_edge_properties(q))
            .unwrap())
    }

    fn set_edge_properties(&self, q: indradb::EdgePropertyQuery, value: &JsonValue) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .run_until(self.async_set_edge_properties(q, value))
            .unwrap();
        Ok(())
    }

    fn delete_edge_properties(&self, q: indradb::EdgePropertyQuery) -> Result<(), indradb::Error> {
        self.exec
            .borrow_mut()
            .run_until(self.async_delete_edge_properties(q))
            .unwrap();
        Ok(())
    }
}
