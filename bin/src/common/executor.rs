use autogen;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::{RpcSystem, Server};
use futures::future;
use futures::stream::MergedItem;
use futures::{Future, Stream};
use std::io::Error as IoError;
use indradb::{Datastore, Transaction};
use std::net::SocketAddr;
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};
use tokio_io::AsyncRead;
use errors::Error;
use server;

fn handle_socket<'a>(handle: &Handle, stream: &TcpStream, service: autogen::service::Client) {
    // Ignore errors on setting nodelay, since the server will still function
    stream.set_nodelay(true).ok();

    let (reader, writer) = stream.split();
    let rpc_network = VatNetwork::new(reader, writer, Side::Server, Default::default());
    let rpc_system = RpcSystem::new(Box::new(rpc_network), Some(service.client));
    handle.spawn(rpc_system.map_err(|_| ()));
}

pub fn run<D, T>(addr: SocketAddr, worker_count: usize, datastore: D, shutdown_stream: Box<dyn Stream<Item=(), Error=IoError>>) -> Result<(), Error>
where D: Datastore<Trans = T> + Send + Sync + 'static,
      T: Transaction + Send + Sync + 'static
{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle)?;
    let service = autogen::service::ToClient::new(server::Service::new(datastore, worker_count)).into_client::<Server>();
    let merged = socket.incoming().merge(shutdown_stream);

    let listener = merged.for_each(move |item| {
        match item {
            MergedItem::First((stream, _)) => {
                handle_socket(&handle, stream, service.clone());
            },
            MergedItem::Second(()) => {
                Ok(false)
            },
            MergedItem::Both((stream, _), ()) => {
                handle_socket(&handle, stream, service.clone());
                Ok(false)
            }
        }
    });

    core.run(listener.into_future());
    Ok(())
}
