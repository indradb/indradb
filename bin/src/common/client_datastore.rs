use autogen;
use capnp::Error as CapnpError;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::{twoparty, RpcSystem};
use converters;
use futures::Future;
use indradb;
use serde_json::value::Value as JsonValue;
use std::cell::RefCell;
use std::net::ToSocketAddrs;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use uuid::Uuid;

pub struct ClientDatastore {
    core: Rc<RefCell<Core>>,
    client: autogen::service::Client,
}

impl ClientDatastore {
    pub fn new(port: u16) -> Self {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let addr = format!("127.0.0.1:{}", port).to_socket_addrs().unwrap().next().unwrap();

        for _ in 0..5 {
            if let Ok(stream) = core.run(TcpStream::connect(&addr, &handle)) {
                stream.set_nodelay(true).unwrap();
                let (reader, writer) = stream.split();
                let rpc_network = Box::new(twoparty::VatNetwork::new(
                    reader,
                    writer,
                    Side::Client,
                    Default::default(),
                ));
                let mut rpc_system = RpcSystem::new(rpc_network, None);
                let client: autogen::service::Client = rpc_system.bootstrap(Side::Server);
                handle.spawn(rpc_system.map_err(|_e| ()));

                let req = client.ping_request();
                let res = core.run(req.send().promise).unwrap();

                if res.get().unwrap().get_ready() {
                    return Self {
                        core: Rc::new(RefCell::new(core)),
                        client: client,
                    };
                }
            }

            sleep(Duration::from_secs(1));
        }

        panic!("Could not connect to the server after a few seconds");
    }
}

impl indradb::Datastore for ClientDatastore {
    type Trans = ClientTransaction;

    fn bulk_insert_vertices<I>(&self, items: I) -> Result<(), indradb::Error>
    where I: Iterator<Item=indradb::BulkInsertItem<indradb::Vertex>> {
        // TODO
        unimplemented!();
    }

    fn bulk_insert_edges<I>(&self, items: I) -> Result<(), indradb::Error>
    where I: Iterator<Item=indradb::BulkInsertItem<indradb::EdgeKey>> {
        // TODO
        unimplemented!();
    }

    fn transaction(&self) -> Result<ClientTransaction, indradb::Error> {
        let trans = self.client.transaction_request().send().pipeline.get_transaction();
        Ok(ClientTransaction::new(self.core.clone(), trans))
    }
}

pub struct ClientTransaction {
    core: Rc<RefCell<Core>>,
    trans: RefCell<autogen::transaction::Client>,
}

impl ClientTransaction {
    fn new(core: Rc<RefCell<Core>>, trans: autogen::transaction::Client) -> Self {
        ClientTransaction {
            core: core,
            trans: RefCell::new(trans),
        }
    }
}

impl ClientTransaction {
    fn execute<F, G>(&self, f: F) -> Result<G, indradb::Error>
    where
        F: FnOnce(&mut autogen::transaction::Client) -> Box<Future<Item = G, Error = CapnpError>>,
    {
        let future = f(&mut self.trans.borrow_mut());
        return self
            .core
            .borrow_mut()
            .run(future)
            .map_err(|err| format!("{:?}", err).into());
    }
}

impl indradb::Transaction for ClientTransaction {
    fn create_vertex(&self, v: &indradb::Vertex) -> Result<bool, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.create_vertex_request();
            converters::from_vertex(v, req.get().init_vertex());

            let f = req.send().promise.and_then(move |res| Ok(res.get()?.get_result()));

            Box::new(f)
        })
    }

    fn create_vertex_from_type(&self, t: indradb::Type) -> Result<Uuid, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.create_vertex_from_type_request();
            req.get().set_t(&t.0);

            let f = req.send().promise.and_then(move |res| {
                let bytes = res.get()?.get_result()?;
                Ok(Uuid::from_slice(bytes).unwrap())
            });

            Box::new(f)
        })
    }

    fn get_vertices(&self, q: &indradb::VertexQuery) -> Result<Vec<indradb::Vertex>, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.get_vertices_request();
            converters::from_vertex_query(&q, req.get().init_q());

            let f = req.send().promise.and_then(move |res| {
                let list = res.get()?.get_result()?;
                let list: Result<Vec<indradb::Vertex>, CapnpError> =
                    list.into_iter().map(|reader| converters::to_vertex(&reader)).collect();
                list
            });

            Box::new(f)
        })
    }

    fn delete_vertices(&self, q: &indradb::VertexQuery) -> Result<(), indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.delete_vertices_request();
            converters::from_vertex_query(&q, req.get().init_q());

            let f = req.send().promise.and_then(move |res| {
                res.get()?;
                Ok(())
            });

            Box::new(f)
        })
    }

    fn get_vertex_count(&self) -> Result<u64, indradb::Error> {
        self.execute(move |trans| {
            let req = trans.get_vertex_count_request();

            let f = req.send().promise.and_then(move |res| Ok(res.get()?.get_result()));

            Box::new(f)
        })
    }

    fn create_edge(&self, e: &indradb::EdgeKey) -> Result<bool, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.create_edge_request();
            converters::from_edge_key(e, req.get().init_key());

            let f = req.send().promise.and_then(move |res| Ok(res.get()?.get_result()));

            Box::new(f)
        })
    }

    fn get_edges(&self, q: &indradb::EdgeQuery) -> Result<Vec<indradb::Edge>, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.get_edges_request();
            converters::from_edge_query(&q, req.get().init_q());

            let f = req.send().promise.and_then(move |res| {
                let list = res.get()?.get_result()?;
                let list: Result<Vec<indradb::Edge>, CapnpError> =
                    list.into_iter().map(|reader| converters::to_edge(&reader)).collect();
                list
            });

            Box::new(f)
        })
    }

    fn delete_edges(&self, q: &indradb::EdgeQuery) -> Result<(), indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.delete_edges_request();
            converters::from_edge_query(&q, req.get().init_q());

            let f = req.send().promise.and_then(move |res| {
                res.get()?;
                Ok(())
            });

            Box::new(f)
        })
    }

    fn get_edge_count(
        &self,
        id: Uuid,
        type_filter: Option<&indradb::Type>,
        direction: indradb::EdgeDirection,
    ) -> Result<u64, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.get_edge_count_request();
            req.get().set_id(id.as_bytes());

            if let Some(type_filter) = type_filter {
                req.get().set_type_filter(&type_filter.0);
            }

            req.get().set_direction(converters::from_edge_direction(direction));

            let f = req.send().promise.and_then(move |res| Ok(res.get()?.get_result()));

            Box::new(f)
        })
    }

    fn get_vertex_properties(
        &self,
        q: &indradb::VertexQuery,
        name: &str,
    ) -> Result<Vec<indradb::VertexProperty>, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.get_vertex_properties_request();
            converters::from_vertex_query(&q, req.get().init_q());
            req.get().set_name(name);

            let f = req.send().promise.and_then(move |res| {
                let list = res.get()?.get_result()?;
                let list: Result<Vec<indradb::VertexProperty>, CapnpError> = list
                    .into_iter()
                    .map(|reader| converters::to_vertex_property(&reader))
                    .collect();
                list
            });

            Box::new(f)
        })
    }

    fn set_vertex_properties(
        &self,
        q: &indradb::VertexQuery,
        name: &str,
        value: &JsonValue,
    ) -> Result<(), indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.set_vertex_properties_request();
            converters::from_vertex_query(&q, req.get().init_q());
            req.get().set_name(name);
            req.get().set_value(&value.to_string());

            let f = req.send().promise.and_then(move |res| {
                res.get()?;
                Ok(())
            });

            Box::new(f)
        })
    }

    fn delete_vertex_properties(&self, q: &indradb::VertexQuery, name: &str) -> Result<(), indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.delete_vertex_properties_request();
            converters::from_vertex_query(&q, req.get().init_q());
            req.get().set_name(name);

            let f = req.send().promise.and_then(move |res| {
                res.get()?;
                Ok(())
            });

            Box::new(f)
        })
    }

    fn get_edge_properties(
        &self,
        q: &indradb::EdgeQuery,
        name: &str,
    ) -> Result<Vec<indradb::EdgeProperty>, indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.get_edge_properties_request();
            converters::from_edge_query(&q, req.get().init_q());
            req.get().set_name(name);

            let f = req.send().promise.and_then(move |res| {
                let list = res.get()?.get_result()?;
                let list: Result<Vec<indradb::EdgeProperty>, CapnpError> = list
                    .into_iter()
                    .map(|reader| converters::to_edge_property(&reader))
                    .collect();
                list
            });

            Box::new(f)
        })
    }

    fn set_edge_properties(&self, q: &indradb::EdgeQuery, name: &str, value: &JsonValue) -> Result<(), indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.set_edge_properties_request();
            converters::from_edge_query(&q, req.get().init_q());
            req.get().set_name(name);
            req.get().set_value(&value.to_string());

            let f = req.send().promise.and_then(move |res| {
                res.get()?;
                Ok(())
            });

            Box::new(f)
        })
    }

    fn delete_edge_properties(&self, q: &indradb::EdgeQuery, name: &str) -> Result<(), indradb::Error> {
        self.execute(move |trans| {
            let mut req = trans.delete_edge_properties_request();
            converters::from_edge_query(&q, req.get().init_q());
            req.get().set_name(name);

            let f = req.send().promise.and_then(move |res| {
                res.get()?;
                Ok(())
            });

            Box::new(f)
        })
    }
}
