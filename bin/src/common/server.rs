use autogen;
use capnp;
use capnp::capability::Promise;
use capnp::Error as CapnpError;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::{RpcSystem, Server};
use converters;
use errors;
use futures::{Future, Stream};
use futures_cpupool::CpuPool;
use indradb;
use indradb::{
    Datastore as IndraDbDatastore, Edge, EdgeProperty, MemoryDatastore, RocksdbDatastore,
    Transaction as IndraDbTransaction, Type, Vertex, VertexProperty,
};
use serde_json;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use uuid::Uuid;

macro_rules! pry_user {
    ($e:expr) => {
        pry!(map_err!($e))
    };
}

struct Service<D: IndraDbDatastore<Trans = T>, T: IndraDbTransaction + Send + Sync + 'static> {
    datastore: D,
    pool: CpuPool,
}

impl<D: IndraDbDatastore<Trans = T>, T: IndraDbTransaction + Send + Sync + 'static> Service<D, T> {
    fn new(datastore: D, worker_count: usize) -> Self {
        Self {
            datastore: datastore,
            pool: CpuPool::new(worker_count),
        }
    }
}

impl<D: IndraDbDatastore<Trans = T>, T: IndraDbTransaction + Send + Sync + 'static> autogen::service::Server
    for Service<D, T>
{
    fn ping(
        &mut self,
        _: autogen::service::PingParams,
        mut res: autogen::service::PingResults,
    ) -> Promise<(), CapnpError> {
        res.get().set_ready(true);
        Promise::ok(())
    }

    fn transaction(
        &mut self,
        _: autogen::service::TransactionParams,
        mut res: autogen::service::TransactionResults,
    ) -> Promise<(), CapnpError> {
        let trans = pry_user!(self.datastore.transaction());
        let trans_server = Transaction::new(self.pool.clone(), trans);
        let trans_client = autogen::transaction::ToClient::new(trans_server).from_server::<Server>();
        res.get().set_transaction(trans_client);
        Promise::ok(())
    }
}

struct Transaction<T: IndraDbTransaction + Send + Sync + 'static> {
    pool: CpuPool,
    trans: Arc<T>,
}

impl<T: IndraDbTransaction + Send + Sync + 'static> Transaction<T> {
    fn new(pool: CpuPool, trans: T) -> Self {
        Self {
            pool: pool,
            trans: Arc::new(trans),
        }
    }
}

impl<T: IndraDbTransaction + Send + Sync + 'static> autogen::transaction::Server for Transaction<T> {
    fn create_vertex(
        &mut self,
        req: autogen::transaction::CreateVertexParams,
        mut res: autogen::transaction::CreateVertexResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_vertex = pry!(pry!(req.get()).get_vertex());
        let vertex = pry!(converters::to_vertex(&cnp_vertex));

        let f = self
            .pool
            .spawn_fn(move || -> Result<bool, CapnpError> { map_err!(trans.create_vertex(&vertex)) })
            .and_then(move |created| -> Result<(), CapnpError> {
                res.get().set_result(created);
                Ok(())
            });

        Promise::from_future(f)
    }

    fn create_vertex_from_type(
        &mut self,
        req: autogen::transaction::CreateVertexFromTypeParams,
        mut res: autogen::transaction::CreateVertexFromTypeResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_t = pry!(pry!(req.get()).get_t());
        let t = pry!(map_err!(indradb::Type::new(cnp_t.to_string())));

        let f = self
            .pool
            .spawn_fn(move || -> Result<Uuid, CapnpError> { map_err!(trans.create_vertex_from_type(t)) })
            .and_then(move |id| -> Result<(), CapnpError> {
                res.get().set_result(id.as_bytes());
                Ok(())
            });

        Promise::from_future(f)
    }

    fn get_vertices(
        &mut self,
        req: autogen::transaction::GetVerticesParams,
        mut res: autogen::transaction::GetVerticesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));

        let f = self
            .pool
            .spawn_fn(move || -> Result<Vec<Vertex>, CapnpError> { map_err!(trans.get_vertices(&q)) })
            .and_then(move |vertices| -> Result<(), CapnpError> {
                let mut res = res.get().init_result(vertices.len() as u32);

                for (i, vertex) in vertices.into_iter().enumerate() {
                    converters::from_vertex(&vertex, res.reborrow().get(i as u32));
                }

                Ok(())
            });

        Promise::from_future(f)
    }

    fn delete_vertices(
        &mut self,
        req: autogen::transaction::DeleteVerticesParams,
        mut res: autogen::transaction::DeleteVerticesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));

        let f = self
            .pool
            .spawn_fn(move || -> Result<(), CapnpError> {
                map_err!(trans.delete_vertices(&q))?;
                Ok(())
            })
            .and_then(move |_| -> Result<(), CapnpError> {
                res.get().set_result(());
                Ok(())
            });

        Promise::from_future(f)
    }

    fn get_vertex_count(
        &mut self,
        _: autogen::transaction::GetVertexCountParams,
        mut res: autogen::transaction::GetVertexCountResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();

        let f = self
            .pool
            .spawn_fn(move || -> Result<u64, CapnpError> { map_err!(trans.get_vertex_count()) })
            .and_then(move |count| -> Result<(), CapnpError> {
                res.get().set_result(count);
                Ok(())
            });

        Promise::from_future(f)
    }

    fn create_edge(
        &mut self,
        req: autogen::transaction::CreateEdgeParams,
        mut res: autogen::transaction::CreateEdgeResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_edge_key = pry!(pry!(req.get()).get_key());
        let edge_key = pry!(converters::to_edge_key(&cnp_edge_key));

        let f = self
            .pool
            .spawn_fn(move || -> Result<bool, CapnpError> { map_err!(trans.create_edge(&edge_key)) })
            .and_then(move |created| -> Result<(), CapnpError> {
                res.get().set_result(created);
                Ok(())
            });

        Promise::from_future(f)
    }

    fn get_edges(
        &mut self,
        req: autogen::transaction::GetEdgesParams,
        mut res: autogen::transaction::GetEdgesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));

        let f = self
            .pool
            .spawn_fn(move || -> Result<Vec<Edge>, CapnpError> { map_err!(trans.get_edges(&q)) })
            .and_then(move |edges| -> Result<(), CapnpError> {
                let mut res = res.get().init_result(edges.len() as u32);

                for (i, edge) in edges.into_iter().enumerate() {
                    converters::from_edge(&edge, res.reborrow().get(i as u32))?;
                }

                Ok(())
            });

        Promise::from_future(f)
    }

    fn delete_edges(
        &mut self,
        req: autogen::transaction::DeleteEdgesParams,
        mut res: autogen::transaction::DeleteEdgesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));

        let f = self
            .pool
            .spawn_fn(move || -> Result<(), CapnpError> {
                map_err!(trans.delete_edges(&q))?;
                Ok(())
            })
            .and_then(move |_| -> Result<(), CapnpError> {
                res.get().set_result(());
                Ok(())
            });

        Promise::from_future(f)
    }

    fn get_edge_count(
        &mut self,
        req: autogen::transaction::GetEdgeCountParams,
        mut res: autogen::transaction::GetEdgeCountResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let id = pry!(map_err!(Uuid::from_slice(pry!(params.get_id()))));
        let type_filter = match pry!(params.get_type_filter()) {
            "" => None,
            value => Some(pry!(map_err!(Type::new(value.to_string())))),
        };
        let converter = converters::to_edge_direction(pry!(params.get_direction()));

        let f = self
            .pool
            .spawn_fn(move || -> Result<u64, CapnpError> {
                map_err!(trans.get_edge_count(id, type_filter.as_ref(), converter))
            })
            .and_then(move |count| -> Result<(), CapnpError> {
                res.get().set_result(count);
                Ok(())
            });

        Promise::from_future(f)
    }

    fn get_vertex_properties(
        &mut self,
        req: autogen::transaction::GetVertexPropertiesParams,
        mut res: autogen::transaction::GetVertexPropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));
        let name = pry!(params.get_name()).to_string();

        let f = self
            .pool
            .spawn_fn(move || -> Result<Vec<VertexProperty>, CapnpError> {
                map_err!(trans.get_vertex_properties(&q, &name))
            })
            .and_then(move |properties| -> Result<(), CapnpError> {
                let mut res = res.get().init_result(properties.len() as u32);

                for (i, property) in properties.into_iter().enumerate() {
                    converters::from_vertex_property(&property, res.reborrow().get(i as u32));
                }

                Ok(())
            });

        Promise::from_future(f)
    }

    fn set_vertex_properties(
        &mut self,
        req: autogen::transaction::SetVertexPropertiesParams,
        mut res: autogen::transaction::SetVertexPropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));
        let name = pry!(params.get_name()).to_string();
        let cnp_value = pry!(params.get_value());
        let value = pry!(map_err!(serde_json::from_str(cnp_value)));

        let f = self
            .pool
            .spawn_fn(move || -> Result<(), CapnpError> { map_err!(trans.set_vertex_properties(&q, &name, &value)) })
            .and_then(move |_| -> Result<(), CapnpError> {
                res.get().set_result(());
                Ok(())
            });

        Promise::from_future(f)
    }

    fn delete_vertex_properties(
        &mut self,
        req: autogen::transaction::DeleteVertexPropertiesParams,
        mut res: autogen::transaction::DeleteVertexPropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));
        let name = pry!(params.get_name()).to_string();

        let f = self
            .pool
            .spawn_fn(move || -> Result<(), CapnpError> { map_err!(trans.delete_vertex_properties(&q, &name)) })
            .and_then(move |_| -> Result<(), CapnpError> {
                res.get().set_result(());
                Ok(())
            });

        Promise::from_future(f)
    }

    fn get_edge_properties(
        &mut self,
        req: autogen::transaction::GetEdgePropertiesParams,
        mut res: autogen::transaction::GetEdgePropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));
        let name = pry!(params.get_name()).to_string();

        let f = self
            .pool
            .spawn_fn(move || -> Result<Vec<EdgeProperty>, CapnpError> { map_err!(trans.get_edge_properties(&q, &name)) })
            .and_then(move |properties| -> Result<(), CapnpError> {
                let mut res = res.get().init_result(properties.len() as u32);

                for (i, property) in properties.into_iter().enumerate() {
                    converters::from_edge_property(&property, res.reborrow().get(i as u32));
                }

                Ok(())
            });

        Promise::from_future(f)
    }

    fn set_edge_properties(
        &mut self,
        req: autogen::transaction::SetEdgePropertiesParams,
        mut res: autogen::transaction::SetEdgePropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));
        let name = pry!(params.get_name()).to_string();
        let cnp_value = pry!(params.get_value());
        let value = pry!(map_err!(serde_json::from_str(cnp_value)));

        let f = self
            .pool
            .spawn_fn(move || -> Result<(), CapnpError> { map_err!(trans.set_edge_properties(&q, &name, &value)) })
            .and_then(move |_| -> Result<(), CapnpError> {
                res.get().set_result(());
                Ok(())
            });

        Promise::from_future(f)
    }

    fn delete_edge_properties(
        &mut self,
        req: autogen::transaction::DeleteEdgePropertiesParams,
        mut res: autogen::transaction::DeleteEdgePropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));
        let name = pry!(params.get_name()).to_string();

        let f = self
            .pool
            .spawn_fn(move || -> Result<(), CapnpError> { map_err!(trans.delete_edge_properties(&q, &name)) })
            .and_then(move |_| -> Result<(), CapnpError> {
                res.get().set_result(());
                Ok(())
            });

        Promise::from_future(f)
    }
}

fn run<D, T>(addr: SocketAddr, datastore: D, worker_count: usize) -> Result<(), errors::Error>
where
    D: IndraDbDatastore<Trans = T> + 'static,
    T: IndraDbTransaction + Send + Sync + 'static,
{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle)?;

    let service = autogen::service::ToClient::new(Service::new(datastore, worker_count)).from_server::<Server>();

    let done = socket.incoming().for_each(move |(socket, _)| {
        socket.set_nodelay(true)?;
        let (reader, writer) = socket.split();
        let rpc_network = VatNetwork::new(reader, writer, Side::Server, Default::default());
        let rpc_system = RpcSystem::new(Box::new(rpc_network), Some(service.clone().client));
        handle.spawn(rpc_system.map_err(|_| ()));
        Ok(())
    });

    core.run(done).unwrap();
    Ok(())
}

pub fn start(binding: &str, connection_string: &str, worker_count: usize) -> Result<(), errors::Error> {
    let addr = binding
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| -> errors::Error { "Could not parse binding".into() })?;

    if connection_string.starts_with("rocksdb://") {
        let path = &connection_string[10..connection_string.len()];

        let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or_else(|_| "512".to_string());
        let max_open_files = max_open_files_str.parse::<i32>().expect(
            "Could not parse environment variable `ROCKSDB_MAX_OPEN_FILES`: must be an \
             i32",
        );

        let datastore = RocksdbDatastore::new(path, Some(max_open_files))
            .expect("Expected to be able to create the RocksDB datastore");

        run(addr, datastore, worker_count)
    } else if connection_string == "memory://" {
        let datastore = MemoryDatastore::default();
        run(addr, datastore, worker_count)
    } else {
        panic!("Cannot parse environment variable `DATABASE_URL`");
    }
}
