use std::convert::TryInto;
use std::pin::Pin;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_stream::wrappers::{ReceiverStream, TcpListenerStream};
use tokio_stream::{Stream, StreamExt};
use tonic::transport::{Error as TonicTransportError, Server as TonicServer};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

const CHANNEL_CAPACITY: usize = 100;

macro_rules! send {
    ($tx:expr, $res:expr) => {
        if let Err(err) = $tx.send($res).await {
            eprintln!("could not send message to client: {}", err);
        }
    };
}

fn map_indradb_result<T>(res: Result<T, indradb::Error>) -> Result<T, Status> {
    res.map_err(|err| Status::internal(format!("{}", err)))
}

fn map_conversion_result<T>(res: Result<T, crate::ConversionError>) -> Result<T, Status> {
    res.map_err(|err| Status::invalid_argument(format!("{}", err)))
}

/// The IndraDB server implementation.
#[derive(Clone)]
pub struct Server<
    D: indradb::Datastore<Trans = T> + Send + Sync + 'static,
    T: indradb::Transaction + Send + Sync + 'static,
> {
    datastore: Arc<D>,
}

#[tonic::async_trait]
impl<D: indradb::Datastore<Trans = T> + Send + Sync + 'static, T: indradb::Transaction + Send + Sync + 'static>
    crate::indra_db_server::IndraDb for Server<D, T>
{
    async fn ping(&self, _: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn sync(&self, _: Request<()>) -> Result<Response<()>, Status> {
        map_indradb_result(self.datastore.sync())?;
        Ok(Response::new(()))
    }

    async fn bulk_insert(&self, request: Request<Streaming<crate::BulkInsertItem>>) -> Result<Response<()>, Status> {
        let items = {
            let mut stream = request.into_inner();
            let (lower_bound_stream_size, _) = stream.size_hint();
            let mut items = Vec::<indradb::BulkInsertItem>::with_capacity(lower_bound_stream_size);
            while let Some(request) = stream.next().await {
                items.push(map_conversion_result(request?.try_into())?);
            }

            items
        };

        let datastore = self.datastore.clone();
        map_indradb_result(datastore.bulk_insert(items.into_iter()))?;
        Ok(Response::new(()))
    }

    type TransactionStream =
        Pin<Box<dyn Stream<Item = Result<crate::TransactionResponse, Status>> + Send + Sync + 'static>>;
    async fn transaction(
        &self,
        request: Request<Streaming<crate::TransactionRequest>>,
    ) -> Result<Response<Self::TransactionStream>, Status> {
        let mut input_stream = request.into_inner();
        let trans = map_indradb_result(self.datastore.clone().transaction())?;
        let (mut tx, rx) = mpsc::channel(CHANNEL_CAPACITY);

        tokio::spawn(async move {
            while let Some(Ok(request)) = input_stream.next().await {
                let request_id = request.request_id;
                if let Some(request) = request.request {
                    if let Err(err) = route(&mut tx, &trans, request_id, request).await {
                        send!(tx, Err(err));
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn index_property(&self, request: Request<crate::IndexPropertyRequest>) -> Result<Response<()>, Status> {
        let name: indradb::Identifier = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.clone().index_property(name))?;
        Ok(Response::new(()))
    }
}

async fn route<T>(
    tx: &mut mpsc::Sender<Result<crate::TransactionResponse, Status>>,
    trans: &T,
    request_id: u32,
    request: crate::TransactionRequestVariant,
) -> Result<(), Status>
where
    T: indradb::Transaction,
{
    let create_response =
        move |response: crate::TransactionResponseVariant| -> Result<crate::TransactionResponse, Status> {
            Ok(crate::TransactionResponse {
                request_id,
                response: Some(response),
            })
        };

    let create_empty_response = move || -> Result<crate::TransactionResponse, Status> {
        create_response(crate::TransactionResponseVariant::Empty(()))
    };

    let create_ok_response = move |ok: bool| -> Result<crate::TransactionResponse, Status> {
        create_response(crate::TransactionResponseVariant::Ok(ok))
    };

    let create_count_response = move |count: u64| -> Result<crate::TransactionResponse, Status> {
        create_response(crate::TransactionResponseVariant::Count(count))
    };

    let create_id_response = move |id: Uuid| -> Result<crate::TransactionResponse, Status> {
        create_response(crate::TransactionResponseVariant::Id(id.into()))
    };

    let create_vertex_response = move |vertex: indradb::Vertex| -> Result<crate::TransactionResponse, Status> {
        create_response(crate::TransactionResponseVariant::Vertex(vertex.into()))
    };

    let create_edge_response = move |edge: indradb::Edge| -> Result<crate::TransactionResponse, Status> {
        create_response(crate::TransactionResponseVariant::Edge(edge.into()))
    };

    let create_vertex_property_response =
        move |vertex_property: indradb::VertexProperty| -> Result<crate::TransactionResponse, Status> {
            create_response(crate::TransactionResponseVariant::VertexProperty(
                vertex_property.into(),
            ))
        };

    let create_vertex_properties_response =
        move |vertex_properties: indradb::VertexProperties| -> Result<crate::TransactionResponse, Status> {
            create_response(crate::TransactionResponseVariant::VertexProperties(
                vertex_properties.into(),
            ))
        };

    let create_edge_property_response =
        move |edge_property: indradb::EdgeProperty| -> Result<crate::TransactionResponse, Status> {
            create_response(crate::TransactionResponseVariant::EdgeProperty(edge_property.into()))
        };

    let create_edge_properties_response =
        move |edge_properties: indradb::EdgeProperties| -> Result<crate::TransactionResponse, Status> {
            create_response(crate::TransactionResponseVariant::EdgeProperties(
                edge_properties.into(),
            ))
        };

    match request {
        crate::TransactionRequestVariant::CreateVertex(request) => {
            let vertex = map_conversion_result(request.try_into())?;
            let res = map_indradb_result(trans.create_vertex(&vertex))?;
            send!(tx, create_ok_response(res));
        }
        crate::TransactionRequestVariant::CreateVertexFromType(request) => {
            let t = map_conversion_result(request.try_into())?;
            let res = map_indradb_result(trans.create_vertex_from_type(t))?;
            send!(tx, create_id_response(res));
        }
        crate::TransactionRequestVariant::GetVertices(request) => {
            let q: indradb::VertexQuery = map_conversion_result(request.try_into())?;
            for vertex in map_indradb_result(trans.get_vertices(q))? {
                send!(tx, create_vertex_response(vertex));
            }
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::DeleteVertices(request) => {
            let q: indradb::VertexQuery = map_conversion_result(request.try_into())?;
            map_indradb_result(trans.delete_vertices(q))?;
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::GetVertexCount(_) => {
            let res = map_indradb_result(trans.get_vertex_count())?;
            send!(tx, create_count_response(res));
        }
        crate::TransactionRequestVariant::CreateEdge(request) => {
            let key = map_conversion_result(request.try_into())?;
            let res = map_indradb_result(trans.create_edge(&key))?;
            send!(tx, create_ok_response(res));
        }
        crate::TransactionRequestVariant::GetEdges(request) => {
            let q: indradb::EdgeQuery = map_conversion_result(request.try_into())?;
            for edge in map_indradb_result(trans.get_edges(q))? {
                send!(tx, create_edge_response(edge));
            }
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::DeleteEdges(request) => {
            let q: indradb::EdgeQuery = map_conversion_result(request.try_into())?;
            map_indradb_result(trans.delete_edges(q))?;
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::GetEdgeCount(request) => {
            let (id, t, direction) = map_conversion_result(request.try_into())?;
            let res = map_indradb_result(trans.get_edge_count(id, t.as_ref(), direction))?;
            send!(tx, create_count_response(res));
        }
        crate::TransactionRequestVariant::GetVertexProperties(request) => {
            let q = map_conversion_result(request.try_into())?;
            for vertex_property in map_indradb_result(trans.get_vertex_properties(q))? {
                send!(tx, create_vertex_property_response(vertex_property));
            }
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::GetAllVertexProperties(request) => {
            let q: indradb::VertexQuery = map_conversion_result(request.try_into())?;
            for vertex_properties in map_indradb_result(trans.get_all_vertex_properties(q))? {
                send!(tx, create_vertex_properties_response(vertex_properties));
            }
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::SetVertexProperties(request) => {
            let (q, value) = map_conversion_result(request.try_into())?;
            map_indradb_result(trans.set_vertex_properties(q, value))?;
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::DeleteVertexProperties(request) => {
            let q = map_conversion_result(request.try_into())?;
            map_indradb_result(trans.delete_vertex_properties(q))?;
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::GetEdgeProperties(request) => {
            let q = map_conversion_result(request.try_into())?;
            for edge_property in map_indradb_result(trans.get_edge_properties(q))? {
                send!(tx, create_edge_property_response(edge_property));
            }
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::GetAllEdgeProperties(request) => {
            let q: indradb::EdgeQuery = map_conversion_result(request.try_into())?;
            for edge_properties in map_indradb_result(trans.get_all_edge_properties(q))? {
                send!(tx, create_edge_properties_response(edge_properties));
            }
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::SetEdgeProperties(request) => {
            let (q, value) = map_conversion_result(request.try_into())?;
            map_indradb_result(trans.set_edge_properties(q, value))?;
            send!(tx, create_empty_response());
        }
        crate::TransactionRequestVariant::DeleteEdgeProperties(request) => {
            let q = map_conversion_result(request.try_into())?;
            map_indradb_result(trans.delete_edge_properties(q))?;
            send!(tx, create_empty_response());
        }
    };

    Ok(())
}

/// Runs the IndraDB server.
///
/// # Arguments
/// * `datastore`: The underlying datastore to use.
/// * `listener`: The TCP listener to run the gRPC server on.
///
/// # Errors
/// This will return an error if the gRPC fails to start on the given
/// listener.
pub async fn run<D, T>(datastore: Arc<D>, listener: TcpListener) -> Result<(), TonicTransportError>
where
    D: indradb::Datastore<Trans = T> + Send + Sync + 'static,
    T: indradb::Transaction + Send + Sync + 'static,
{
    let svc = crate::indra_db_server::IndraDbServer::new(Server { datastore });
    let incoming = TcpListenerStream::new(listener);
    TonicServer::builder()
        .add_service(svc)
        .serve_with_incoming(incoming)
        .await?;
    Ok(())
}
