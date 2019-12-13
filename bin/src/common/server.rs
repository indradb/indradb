use crate::autogen;
use crate::converters;

use std::net::SocketAddr;
use std::sync::Arc;

use capnp::capability::Promise;
use capnp::Error as CapnpError;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::twoparty::VatNetwork;
use capnp_rpc::{RpcSystem, Server};
use futures::prelude::*;
use futures::executor::LocalSpawner;
use futures::task::LocalSpawn;
use indradb;
use indradb::{Datastore as IndraDbDatastore, Transaction as IndraDbTransaction, Type};
use serde_json;
use uuid::Uuid;
use async_std::task::spawn_blocking;
use async_std::net::TcpListener;
use async_std::io::Error as AsyncIoError;

struct Service<D: IndraDbDatastore<Trans = T> + Send + Sync + 'static, T: IndraDbTransaction + Send + Sync + 'static> {
    datastore: Arc<D>,
}

impl<D: IndraDbDatastore<Trans = T> + Send + Sync + 'static, T: IndraDbTransaction + Send + Sync + 'static>
    Service<D, T>
{
    fn new(datastore: D) -> Self {
        Self {
            datastore: Arc::new(datastore),
        }
    }
}

impl<D: IndraDbDatastore<Trans = T> + Send + Sync + 'static, T: IndraDbTransaction + Send + Sync + 'static>
    autogen::service::Server for Service<D, T>
{
    fn ping(
        &mut self,
        _: autogen::service::PingParams,
        mut res: autogen::service::PingResults,
    ) -> Promise<(), CapnpError> {
        res.get().set_ready(true);
        Promise::ok(())
    }

    fn bulk_insert(
        &mut self,
        req: autogen::service::BulkInsertParams,
        mut res: autogen::service::BulkInsertResults,
    ) -> Promise<(), CapnpError> {
        let datastore = self.datastore.clone();
        let cnp_items = pry!(pry!(req.get()).get_items());
        let items = pry!(converters::to_bulk_insert_items(&cnp_items));

        Promise::from_future(async move {
            spawn_blocking(move || {
                converters::map_capnp_err(datastore.bulk_insert(items))
            }).await?;
            res.get().set_result(());
            Ok(())
        })
    }

    fn transaction(
        &mut self,
        _: autogen::service::TransactionParams,
        mut res: autogen::service::TransactionResults,
    ) -> Promise<(), CapnpError> {
        let trans = pry!(converters::map_capnp_err(self.datastore.transaction()));
        let trans_server = Transaction::new(trans);
        let trans_client = autogen::transaction::ToClient::new(trans_server).into_client::<Server>();
        res.get().set_transaction(trans_client);
        Promise::ok(())
    }
}

struct Transaction<T: IndraDbTransaction + Send + Sync + 'static> {
    trans: Arc<T>,
}

impl<T: IndraDbTransaction + Send + Sync + 'static> Transaction<T> {
    fn new(trans: T) -> Self {
        Self {
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

        Promise::from_future(async move {
            let created = spawn_blocking(move || {
                converters::map_capnp_err(trans.create_vertex(&vertex))
            }).await?;
            res.get().set_result(created);
            Ok(())
        })
    }

    fn create_vertex_from_type(
        &mut self,
        req: autogen::transaction::CreateVertexFromTypeParams,
        mut res: autogen::transaction::CreateVertexFromTypeResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_t = pry!(pry!(req.get()).get_t());
        let t = pry!(converters::map_capnp_err(indradb::Type::new(cnp_t)));

        Promise::from_future(async move {
            let id = spawn_blocking(move || {
                converters::map_capnp_err(trans.create_vertex_from_type(t))
            }).await?;
            res.get().set_result(id.as_bytes());
            Ok(())
        })
    }

    fn get_vertices(
        &mut self,
        req: autogen::transaction::GetVerticesParams,
        mut res: autogen::transaction::GetVerticesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));

        Promise::from_future(async move {
            let vertices = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_vertices(q))
            }).await?;
            
            let mut res = res.get().init_result(vertices.len() as u32);

            for (i, vertex) in vertices.into_iter().enumerate() {
                converters::from_vertex(&vertex, res.reborrow().get(i as u32));
            }

            Ok(())
        })
    }

    fn delete_vertices(
        &mut self,
        req: autogen::transaction::DeleteVerticesParams,
        mut res: autogen::transaction::DeleteVerticesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));

        Promise::from_future(async move {
            spawn_blocking(move || {
                converters::map_capnp_err(trans.delete_vertices(q))
            }).await?;
            res.get().set_result(());
            Ok(())
        })
    }

    fn get_vertex_count(
        &mut self,
        _: autogen::transaction::GetVertexCountParams,
        mut res: autogen::transaction::GetVertexCountResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();

        Promise::from_future(async move {
            let count = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_vertex_count())
            }).await?;
            res.get().set_result(count);
            Ok(())
        })
    }

    fn create_edge(
        &mut self,
        req: autogen::transaction::CreateEdgeParams,
        mut res: autogen::transaction::CreateEdgeResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_edge_key = pry!(pry!(req.get()).get_key());
        let edge_key = pry!(converters::to_edge_key(&cnp_edge_key));

        Promise::from_future(async move {
            let created = spawn_blocking(move || {
                converters::map_capnp_err(trans.create_edge(&edge_key))
            }).await?;
            res.get().set_result(created);
            Ok(())
        })
    }

    fn get_edges(
        &mut self,
        req: autogen::transaction::GetEdgesParams,
        mut res: autogen::transaction::GetEdgesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));

        Promise::from_future(async move {
            let edges = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_edges(q))
            }).await?;
            
            let mut res = res.get().init_result(edges.len() as u32);

            for (i, edge) in edges.into_iter().enumerate() {
                converters::from_edge(&edge, res.reborrow().get(i as u32))?;
            }

            Ok(())
        })
    }

    fn delete_edges(
        &mut self,
        req: autogen::transaction::DeleteEdgesParams,
        mut res: autogen::transaction::DeleteEdgesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));

        Promise::from_future(async move {
            spawn_blocking(move || {
                converters::map_capnp_err(trans.delete_edges(q))
            }).await?;
            res.get().set_result(());
            Ok(())
        })
    }

    fn get_edge_count(
        &mut self,
        req: autogen::transaction::GetEdgeCountParams,
        mut res: autogen::transaction::GetEdgeCountResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let id = pry!(converters::map_capnp_err(Uuid::from_slice(pry!(params.get_id()))));
        let t = match pry!(params.get_t()) {
            "" => None,
            value => Some(pry!(converters::map_capnp_err(Type::new(value)))),
        };
        let converter = converters::to_edge_direction(pry!(params.get_direction()));

        Promise::from_future(async move {
            let count = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_edge_count(id, t.as_ref(), converter))
            }).await?;
            res.get().set_result(count);
            Ok(())
        })
    }

    fn get_vertex_properties(
        &mut self,
        req: autogen::transaction::GetVertexPropertiesParams,
        mut res: autogen::transaction::GetVertexPropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_vertex_property_query(&cnp_q));

        Promise::from_future(async move {
            let properties = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_vertex_properties(q))
            }).await?;
            
            let mut res = res.get().init_result(properties.len() as u32);

            for (i, property) in properties.into_iter().enumerate() {
                converters::from_vertex_property(&property, res.reborrow().get(i as u32));
            }

            Ok(())
        })
    }

    fn get_all_vertex_properties(
        &mut self,
        req: autogen::transaction::GetAllVertexPropertiesParams,
        mut res: autogen::transaction::GetAllVertexPropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_vertex_query(&cnp_q));

        Promise::from_future(async move {
            let vertex_props = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_all_vertex_properties(q))
            }).await?;
            
            let mut res = res.get().init_result(vertex_props.len() as u32);

            for (i, vertex) in vertex_props.into_iter().enumerate() {
                converters::from_vertex_properties(&vertex, &mut res.reborrow().get(i as u32));
            }
            Ok(())
        })
    }

    fn set_vertex_properties(
        &mut self,
        req: autogen::transaction::SetVertexPropertiesParams,
        mut res: autogen::transaction::SetVertexPropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_vertex_property_query(&cnp_q));
        let cnp_value = pry!(params.get_value());
        let value = pry!(converters::map_capnp_err(serde_json::from_str(cnp_value)));

        Promise::from_future(async move {
            spawn_blocking(move || {
                converters::map_capnp_err(trans.set_vertex_properties(q, &value))
            }).await?;
            res.get().set_result(());
            Ok(())
        })
    }

    fn delete_vertex_properties(
        &mut self,
        req: autogen::transaction::DeleteVertexPropertiesParams,
        mut res: autogen::transaction::DeleteVertexPropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_vertex_property_query(&cnp_q));

        Promise::from_future(async move {
            spawn_blocking(move || {
                converters::map_capnp_err(trans.delete_vertex_properties(q))
            }).await?;
            res.get().set_result(());
            Ok(())
        })
    }

    fn get_edge_properties(
        &mut self,
        req: autogen::transaction::GetEdgePropertiesParams,
        mut res: autogen::transaction::GetEdgePropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_edge_property_query(&cnp_q));

        Promise::from_future(async move {
            let properties = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_edge_properties(q))
            }).await?;
            
            let mut res = res.get().init_result(properties.len() as u32);

            for (i, property) in properties.into_iter().enumerate() {
                converters::from_edge_property(&property, res.reborrow().get(i as u32));
            }

            Ok(())
        })
    }

    fn get_all_edge_properties(
        &mut self,
        req: autogen::transaction::GetAllEdgePropertiesParams,
        mut res: autogen::transaction::GetAllEdgePropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let cnp_q = pry!(pry!(req.get()).get_q());
        let q = pry!(converters::to_edge_query(&cnp_q));

        Promise::from_future(async move {
            let edge_props = spawn_blocking(move || {
                converters::map_capnp_err(trans.get_all_edge_properties(q))
            }).await?;
            
            let mut res = res.get().init_result(edge_props.len() as u32);

            for (i, edge) in edge_props.into_iter().enumerate() {
                converters::from_edge_properties(&edge, &mut res.reborrow().get(i as u32));
            }

            Ok(())
        })
    }

    fn set_edge_properties(
        &mut self,
        req: autogen::transaction::SetEdgePropertiesParams,
        mut res: autogen::transaction::SetEdgePropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_edge_property_query(&cnp_q));
        let cnp_value = pry!(params.get_value());
        let value = pry!(converters::map_capnp_err(serde_json::from_str(cnp_value)));

        Promise::from_future(async move {
            spawn_blocking(move || {
                converters::map_capnp_err(trans.set_edge_properties(q, &value))
            }).await?;
            res.get().set_result(());
            Ok(())
        })
    }

    fn delete_edge_properties(
        &mut self,
        req: autogen::transaction::DeleteEdgePropertiesParams,
        mut res: autogen::transaction::DeleteEdgePropertiesResults,
    ) -> Promise<(), CapnpError> {
        let trans = self.trans.clone();
        let params = pry!(req.get());
        let cnp_q = pry!(params.get_q());
        let q = pry!(converters::to_edge_property_query(&cnp_q));

        Promise::from_future(async move {
            spawn_blocking(move || {
                converters::map_capnp_err(trans.delete_edge_properties(q))
            }).await?;
            res.get().set_result(());
            Ok(())
        })
    }
}

pub async fn run<D, T>(addr: SocketAddr, datastore: D, spawner: LocalSpawner) -> Result<(), AsyncIoError>
where
    D: IndraDbDatastore<Trans = T> + Send + Sync + 'static,
    T: IndraDbTransaction + Send + Sync + 'static,
{
    let listener = TcpListener::bind(&addr).await?;

    let service = autogen::service::ToClient::new(Service::new(datastore)).into_client::<Server>();

    let mut incoming = listener.incoming();

    while let Some(socket) = incoming.next().await {
        if let Ok(socket) = socket {
            if let Err(_) = socket.set_nodelay(true) {
                eprintln!("could not set nodelay on client");
            }

            let (reader, writer) = socket.split();
            let rpc_network = VatNetwork::new(reader, writer, Side::Server, Default::default());
            let rpc_system = RpcSystem::new(Box::new(rpc_network), Some(service.clone().client));

            spawner.spawn_local_obj(
                Box::pin(rpc_system.map_err(|err| {
                    eprintln!("error handling request: {:?}", err)
                }).map(|_| ())).into()
            ).expect("Expected to be able to spawn a request handler")
        } else {
            eprintln!("connection setup failed");
        }
    }

    Ok(())
}
