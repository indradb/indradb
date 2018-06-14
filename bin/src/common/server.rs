use tokio_core::reactor::{Core, Handle};
use autogen;
use capnp_rpc::{RpcSystem, Server};
use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp::Error as CapnpError;
use capnp::capability::Promise;
use tokio_core::net::{TcpListener};
use std::net::{ToSocketAddrs};
use futures::{Future, Stream};
use proxy_datastore;
use capnp;
use indradb;
use indradb::{Datastore as IndraDbDatastore, Transaction as IndraDbTransaction, Vertex};
use tokio_io::AsyncRead;
use errors;
use converters;
use std::thread;
use std::sync::Arc;
use uuid::Uuid;
use futures_cpupool::CpuPool;
use std::error::Error;
use std::sync::mpsc;

macro_rules! pry_user {
    ($e:expr) => (pry!(map_err!($e)))
}

// TODO: make this configurable
const WORKER_COUNT: usize = 8;

struct Service {
    handle: Handle,
    datastore: proxy_datastore::ProxyDatastore,
    pool: CpuPool
}

impl Service {
    fn new(handle: Handle, datastore: proxy_datastore::ProxyDatastore) -> Self {
        Self {
            handle: handle,
            datastore: datastore,
            pool: CpuPool::new(WORKER_COUNT)
        }
    }
}

impl autogen::service::Server for Service {
    fn ping(&mut self, _: autogen::service::PingParams, mut res: autogen::service::PingResults) -> Promise<(), CapnpError> {
        res.get().set_ready(true);
        Promise::ok(())
    }

    fn transaction(&mut self, _: autogen::service::TransactionParams, mut res: autogen::service::TransactionResults) -> Promise<(), CapnpError> {
        let trans = pry_user!(self.datastore.transaction());
        let trans_server = Transaction::new(self.pool.clone(), trans);
        let trans_client = autogen::transaction::ToClient::new(trans_server).from_server::<Server>();
        res.get().set_transaction(trans_client);
        Promise::ok(())
    }
}

struct Transaction {
    pool: CpuPool,
    trans: Arc<proxy_datastore::ProxyTransaction>
}

impl Transaction {
    fn new(pool: CpuPool, trans: proxy_datastore::ProxyTransaction) -> Self {
        Self {
            pool: pool,
            trans: Arc::new(trans)
        }
    }
}

impl autogen::transaction::Server for Transaction {
    fn create_vertex(&mut self, req: autogen::transaction::CreateVertexParams<>, mut res: autogen::transaction::CreateVertexResults<>) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_vertex = pry!(pry!(req.get()).get_vertex());
        let vertex = pry!(converters::to_vertex(&cnp_vertex));
        
        let f = self.pool.spawn_fn(move || -> Result<bool, CapnpError> {
            map_err!(trans.create_vertex(&vertex))
        }).and_then(move |created| -> Result<(), CapnpError> {
            res.get().set_result(created);
            Ok(())
        });
        Promise::from_future(f)
    }

    fn create_vertex_from_type(&mut self, req: autogen::transaction::CreateVertexFromTypeParams<>, mut res: autogen::transaction::CreateVertexFromTypeResults<>) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_t = pry!(pry!(req.get()).get_t());
        let t = pry!(map_err!(indradb::Type::new(cnp_t.to_string())));
        
        let f = self.pool.spawn_fn(move || -> Result<Uuid, CapnpError> {
            map_err!(trans.create_vertex_from_type(t))
        }).and_then(move |id| -> Result<(), CapnpError> {
            res.get().set_result(id.as_bytes());
            Ok(())
        });
        Promise::from_future(f)
    }

    fn get_vertices(&mut self, req: autogen::transaction::GetVerticesParams<>, mut res: autogen::transaction::GetVerticesResults<>) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));
        
        let f = self.pool.spawn_fn(move || -> Result<Vec<Vertex>, CapnpError> {
            map_err!(trans.get_vertices(&q))
        }).and_then(move |vertices| -> Result<(), CapnpError> {
            let mut res = res.get().init_result(vertices.len() as u32);

            for (i, vertex) in vertices.into_iter().enumerate() {
                let mut cnp_vertex = res.reborrow().get(i as u32);
                converters::from_vertex(vertex, &mut cnp_vertex);
            }

            Ok(())
        });
        Promise::from_future(f)
    }

    fn delete_vertices(&mut self, req: autogen::transaction::DeleteVerticesParams<>, mut res: autogen::transaction::DeleteVerticesResults<>) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));
        
        let f = self.pool.spawn_fn(move || -> Result<(), CapnpError> {
            map_err!(trans.delete_vertices(&q))?;
            Ok(())
        }).and_then(move |_| -> Result<(), CapnpError> {
            res.get().set_result(());
            Ok(())
        });
        Promise::from_future(f)
    }

    fn get_vertex_count(&mut self, req: autogen::transaction::GetVertexCountParams<>, res: autogen::transaction::GetVertexCountResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn create_edge(&mut self, req: autogen::transaction::CreateEdgeParams<>, res: autogen::transaction::CreateEdgeResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn get_edges(&mut self, req: autogen::transaction::GetEdgesParams<>, res: autogen::transaction::GetEdgesResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn delete_edges(&mut self, req: autogen::transaction::DeleteEdgesParams<>, res: autogen::transaction::DeleteEdgesResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn get_edge_count(&mut self, req: autogen::transaction::GetEdgeCountParams<>, res: autogen::transaction::GetEdgeCountResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn get_global_metadata(&mut self, req: autogen::transaction::GetGlobalMetadataParams<>, res: autogen::transaction::GetGlobalMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn set_global_metadata(&mut self, req: autogen::transaction::SetGlobalMetadataParams<>, res: autogen::transaction::SetGlobalMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn delete_global_metadata(&mut self, req: autogen::transaction::DeleteGlobalMetadataParams<>, res: autogen::transaction::DeleteGlobalMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn get_vertex_metadata(&mut self, req: autogen::transaction::GetVertexMetadataParams<>, res: autogen::transaction::GetVertexMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn set_vertex_metadata(&mut self, req: autogen::transaction::SetVertexMetadataParams<>, res: autogen::transaction::SetVertexMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn delete_vertex_metadata(&mut self, req: autogen::transaction::DeleteVertexMetadataParams<>, res: autogen::transaction::DeleteVertexMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn get_edge_metadata(&mut self, req: autogen::transaction::GetEdgeMetadataParams<>, res: autogen::transaction::GetEdgeMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn set_edge_metadata(&mut self, req: autogen::transaction::SetEdgeMetadataParams<>, res: autogen::transaction::SetEdgeMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn delete_edge_metadata(&mut self, req: autogen::transaction::DeleteEdgeMetadataParams<>, res: autogen::transaction::DeleteEdgeMetadataResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }
}

pub fn start(binding: &str) -> Result<(), errors::Error> {
    let datastore = proxy_datastore::datastore();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = binding.to_socket_addrs()?.next().ok_or_else(|| -> errors::Error { "Could not parse binding".into() })?;
    let socket = TcpListener::bind(&addr, &handle)?;
    let service = autogen::service::ToClient::new(Service::new(handle.clone(), datastore)).from_server::<Server>();

    let done = {
        let handle = handle.clone();

        socket.incoming().for_each(move |(socket, _)| {
            socket.set_nodelay(true)?;
            let (reader, writer) = socket.split();
            let handle = handle.clone();
            let rpc_network = VatNetwork::new(reader, writer, Side::Server, Default::default());
            let rpc_system = RpcSystem::new(Box::new(rpc_network), Some(service.clone().client));
            handle.spawn(rpc_system.map_err(|_| ()));
            Ok(())
        })
    };

    core.run(done).unwrap();
    Ok(())
}
