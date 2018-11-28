use autogen;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::{RpcSystem, Server};
use futures::{Future, Stream};
use indradb::{Datastore, Transaction};
use std::net::SocketAddr;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use errors::Error;
use server;

pub fn run<D, T>(addr: SocketAddr, worker_count: usize, datastore: D) -> Result<(), Error>
where D: Datastore<Trans = T> + Send + Sync + 'static,
      T: Transaction + Send + Sync + 'static
{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle)?;
    let service = autogen::service::ToClient::new(server::Service::new(datastore, worker_count)).into_client::<Server>();

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
