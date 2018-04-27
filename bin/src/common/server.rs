use tokio_core::reactor::{Core, Handle};
use autogen;
use errors::{Error, Result};
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
use converters::ErrorableFrom;
use std::thread;
use crossbeam_channel::{Sender, Receiver, bounded};

macro_rules! map_user_err {
    ($e:expr) => ($e.map_err(|err| capnp::Error::failed(err.description().to_string())))
}

macro_rules! pry_user {
    ($e:expr) => (pry!(map_user_err!($e)))
}

const WORKER_CHANNEL_SIZE: usize = 10;

struct Service {
    handle: Handle,
    datastore: proxy_datastore::ProxyDatastore
}

impl Service {
    fn new(handle: Handle, datastore: proxy_datastore::ProxyDatastore) -> Self {
        Self {
            handle: handle,
            datastore: datastore
        }
    }
}

impl autogen::service::Server for Service {
    fn ping(&mut self, _: autogen::service::PingParams, res: autogen::service::PingResults) -> Promise<(), CapnpError> {
        res.get().set_ready(true);
        Promise::ok(())
    }

    fn transaction(&mut self, _: autogen::service::TransactionParams, res: autogen::service::TransactionResults) -> Promise<(), CapnpError> {
        let trans = pry_user!(self.datastore.transaction());
        let trans_server = Transaction::new(self.handle.clone(), &self.datastore);
        let trans_client = autogen::transaction::ToClient::new(trans_server).from_server::<Server>();
        res.get().set_transaction(trans_client);
        Promise::ok(())
    }
}

struct Transaction {
    handle: Handle,
    trans: proxy_datastore::ProxyTransaction,
    sender: Sender<TransactionRequest>,
    worker: thread::JoinHandle<()>
}

impl Transaction {
    fn new(handle: Handle, datastore: &proxy_datastore::ProxyDatastore) -> Self {
        // TODO: this currently sets up a new thread per transaction, which
        // may be heavy-weight if the transactions are short-lived. Explore
        // thread pooling as an alternative.
        let (sender, receiver) = bounded::<TransactionRequest>(WORKER_CHANNEL_SIZE);

        let worker = thread::spawn(|| {
            let trans = datastore.transaction();

            loop {
                match receiver.recv() {
                    TransactionRequest::CreateVertex(vertex, res) => {
                        trans.create_vertex(vertex)
                    },
                    TransactionRequest::TransactionRequest::Shutdown => return,
                    _ => unimplemented!()
                };
            }
        });

        Self {
            handle: handle,
            sender: sender,
            worker: worker
        }
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        self.sender.send(TransactionRequest::Shutdown);
        self.worker.join().unwrap();
    }
}

impl autogen::transaction::Server for Transaction {
    fn create_vertex(&mut self, req: autogen::transaction::CreateVertexParams<>, res: autogen::transaction::CreateVertexResults<>) -> Promise<(), CapnpError> {
        let params = pry!(req.get());
        let vertex = pry_user!(indradb::Vertex::errorable_from(&pry!(params.get_vertex())));
        let f = self.pool.spawn_fn(|| {
            map_user_err!(self.trans.create_vertex(&vertex))?;
            Ok(())
        });
        Promise::from_future(f)
    }

    fn create_vertex_from_type(&mut self, req: autogen::transaction::CreateVertexFromTypeParams<>, res: autogen::transaction::CreateVertexFromTypeResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn get_vertices(&mut self, req: autogen::transaction::GetVerticesParams<>, res: autogen::transaction::GetVerticesResults<>) -> Promise<(), CapnpError> {
        unimplemented!();
    }

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

struct TransactionWorker {
    //
}

enum TransactionRequest {
    Shutdown,
    CreateVertex {
        vertex: indradb::Vertex,
        res: autogen::transaction::CreateVertexFromTypeResults<>
    },
    CreateVertexFromType {
        res: autogen::transaction::CreateVertexFromTypeResults<>
    },
    GetVertices {
        res: autogen::transaction::GetVerticesResults<>
    },
    DeleteVertices {
        res: autogen::transaction::DeleteVerticesResults<>
    },
    GetVertexCount {
        res: autogen::transaction::GetVertexCountResults<>
    },
    CreateEdge {
        res: autogen::transaction::CreateEdgeResults<>
    },
    GetEdges {
        res: autogen::transaction::GetEdgesResults<>
    },
    DeleteEdges {
        res: autogen::transaction::DeleteEdgesResults<>
    },
    GetEdgeCount {
        res: autogen::transaction::GetEdgeCountResults<>
    },
    GetGlobalMetadata {
        res: autogen::transaction::GetGlobalMetadataResults<>
    },
    SetGlobalMetadata {
        res: autogen::transaction::SetGlobalMetadataResults<>
    },
    DeleteGlobalMetadata {
        res: autogen::transaction::DeleteGlobalMetadataResults<>
    },
    GetVertexMetadata {
        res: autogen::transaction::GetVertexMetadataResults<>
    },
    SetVertexMetadata {
        res: autogen::transaction::SetVertexMetadataResults<>
    },
    DeleteVertexMetadata {
        res: autogen::transaction::DeleteVertexMetadataResults<>
    },
    GetEdgeMetadata {
        res: autogen::transaction::GetEdgeMetadataResults<>
    },
    SetEdgeMetadata {
        res: autogen::transaction::SetEdgeMetadataResults<>
    },
    DeleteEdgeMetadata {
        res: autogen::transaction::DeleteEdgeMetadataResults<>
    },
}

pub fn start(binding: &str) -> Result<()> {
    let datastore = proxy_datastore::datastore();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = binding.to_socket_addrs()?.next().ok_or_else(|| -> Error { "Could not parse binding".into() })?;
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
