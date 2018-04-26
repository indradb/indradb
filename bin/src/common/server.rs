use tokio_core::reactor::{Core, Handle};
use autogen;
use errors::{Error, Result};
use capnp_rpc::{RpcSystem, Server};
use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp::Error as CapnpError;
use capnp::capability::Promise;
use tokio_core::net::{TcpStream, TcpListener};
use std::net::{SocketAddr, ToSocketAddrs};
use futures::{Future, Sink, Stream};

use capnp_rpc::twoparty;
use futures::stream::Wait;
use indradb;
use tokio_io::AsyncRead;

struct Service {
    handle: Handle
}

impl Service {
    fn new(handle: Handle) -> Self {
        Self {
            handle: handle
        }
    }
}

impl autogen::service::Server for Service {
    fn ping(&mut self, _: autogen::service::PingParams, _: autogen::service::PingResults) -> Promise<(), CapnpError> {
        unimplemented!();
    }

    fn transaction(&mut self, _: autogen::service::TransactionParams, _: autogen::service::TransactionResults) -> Promise<(), CapnpError> {
        unimplemented!();
    }
}

pub fn start(binding: &str) -> Result<()> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = binding.to_socket_addrs()?.next().ok_or_else(|| -> Error { "Could not parse binding".into() })?;
    let socket = TcpListener::bind(&addr, &handle)?;
    let service = autogen::service::ToClient::new(Service::new(handle.clone())).from_server::<Server>();

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
