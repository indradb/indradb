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
use indradb::{Datastore as IndraDbDatastore, Transaction as IndraDbTransaction};
use tokio_io::AsyncRead;
use errors;
#[macro_use]
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

macro_rules! proxy {
    ($name:ident ( $req_typ:ident, $res_typ:ident, $ch_typ:ty ) => $arg_code:expr => $trans_code:expr => $serializer_code:expr) => (
        fn $name(&mut self, req: autogen::transaction::$req_typ<>, mut res: autogen::transaction::$res_typ<>) -> Promise<(), CapnpError> {
            let trans = self.trans.clone();
            let args = pry!($arg_code(pry!(req.get())));
            let (sender, receiver) = mpsc::sync_channel::<$ch_typ>(1);
            let f = self.pool.spawn_fn(move || -> Result<(), CapnpError> {
                let result: $ch_typ = $trans_code(trans, args)?;
                map_err!(sender.send(result))
            }).and_then(move |_| -> Result<(), CapnpError> {
                let result = map_err!(receiver.recv())?;
                $serializer_code(res, result);
                Ok(())
            });
            Promise::from_future(f)
        }
    )
}

impl autogen::transaction::Server for Transaction {
    proxy!(
        create_vertex(CreateVertexParams, CreateVertexResults, bool)
        => |params: autogen::transaction::create_vertex_params::Reader| {
            let cnp_vertex = params.get_vertex()?;
            converters::to_vertex(&cnp_vertex)
        }
        => |trans: Arc<proxy_datastore::ProxyTransaction>, vertex: indradb::Vertex| -> Result<bool, CapnpError> {
            map_err!(trans.create_vertex(&vertex))
        }
        => |mut res: autogen::transaction::CreateVertexResults, value: bool| {
            res.get().set_result(value);
        }
    );

    proxy!(
        create_vertex_from_type(CreateVertexFromTypeParams, CreateVertexFromTypeResults, Uuid)
        => |params: autogen::transaction::create_vertex_from_type_params::Reader| {
            indradb::Type::new(map_err!(params.get_t())?.to_string())
        }
        => |trans: Arc<proxy_datastore::ProxyTransaction>, t: indradb::Type| -> Result<Uuid, CapnpError> {
            map_err!(trans.create_vertex_from_type(t))
        }
        => |mut res: autogen::transaction::CreateVertexFromTypeResults, value: Uuid| {
            res.get().set_result(value.as_bytes().as_ref());
        }
    );

    proxy!(
        get_vertices(GetVerticesParams, GetVerticesResults, Vec<indradb::Vertex>)
        => |params: autogen::transaction::get_vertices_params::Reader| {
            let cnp_q = params.get_q()?;
            converters::to_vertex_query(&cnp_q)
        }
        => |trans: Arc<proxy_datastore::ProxyTransaction>, q: indradb::VertexQuery| -> Result<Vec<indradb::Vertex>, CapnpError> {
            map_err!(trans.get_vertices(&q))
        }
        => |mut res: autogen::transaction::GetVerticesResults, vertices: Vec<indradb::Vertex>| {
            let mut list = res.get().init_result(vertices.len() as u32);
            
            for (i, vertex) in vertices.into_iter().enumerate() {
                let mut cnp_vertex = list.reborrow().get(i as u32);
                converters::from_vertex(vertex, &mut cnp_vertex);
            }
        }
    );

    // proxy!(
    //     get_vertices(DeleteVerticesParams, DeleteVerticesResults, ())
    //     => |params: autogen::transaction::delete_vertices_params::Reader| {
    //         let cnp_q = params.get_q()?;
    //         map_err!(converters::to_vertex_query(&cnp_q))
    //     }
    //     => |trans: Arc<proxy_datastore::ProxyTransaction>, q: indradb::VertexQuery| -> Result<Vec<indradb::Vertex>, CapnpError> {
    //         map_err!(trans.get_vertices(&q))
    //     }
    //     => |mut res: autogen::transaction::GetVerticesResults, vertices: Vec<indradb::Vertex>| {
    //         let mut list = res.get().init_result(vertices.len() as u32);
            
    //         for (i, vertex) in vertices.into_iter().enumerate() {
    //             let mut cnp_vertex = list.reborrow().get(i as u32);
    //             converters::from_vertex(vertex, &mut cnp_vertex);
    //         }
    //     }
    // );

    fn delete_vertices(&mut self, req: autogen::transaction::DeleteVerticesParams<>, res: autogen::transaction::DeleteVerticesResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
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
