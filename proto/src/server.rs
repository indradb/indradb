use std::convert::TryInto;
use std::pin::Pin;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_stream::wrappers::{ReceiverStream, TcpListenerStream};
use tokio_stream::{Stream, StreamExt};
use tonic::transport::{Error as TonicTransportError, Server as TonicServer};
use tonic::{Request, Response, Status, Streaming};

const CHANNEL_CAPACITY: usize = 100;

async fn send<IT, PT>(tx: mpsc::Sender<Result<PT, Status>>, result: Result<Vec<IT>, indradb::Error>)
where
    IT: Into<PT>,
{
    match map_indradb_result(result) {
        Ok(values) => {
            for value in values {
                if let Err(err) = tx.send(Ok(value.into())).await {
                    eprintln!("could not send message to client: {}", err);
                }
            }
        }
        Err(err) => {
            if let Err(err) = tx.send(Err(err)).await {
                eprintln!("could not send message to client: {}", err);
            }
        }
    }
}

fn map_indradb_result<T>(res: Result<T, indradb::Error>) -> Result<T, Status> {
    res.map_err(|err| Status::internal(format!("{}", err)))
}

fn map_conversion_result<T>(res: Result<T, crate::ConversionError>) -> Result<T, Status> {
    res.map_err(|err| Status::invalid_argument(format!("{}", err)))
}

/// The IndraDB server implementation.
#[derive(Clone)]
pub struct Server<D: indradb::Datastore + Send + Sync + 'static> {
    datastore: Arc<D>,
}

#[tonic::async_trait]
impl<D: indradb::Datastore + Send + Sync + 'static> crate::indra_db_server::IndraDb for Server<D> {
    async fn ping(&self, _: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn sync(&self, _: Request<()>) -> Result<Response<()>, Status> {
        map_indradb_result(self.datastore.sync())?;
        Ok(Response::new(()))
    }

    async fn create_vertex(&self, request: Request<crate::Vertex>) -> Result<Response<crate::CreateResponse>, Status> {
        let vertex = map_conversion_result(request.into_inner().try_into())?;
        let res = map_indradb_result(self.datastore.create_vertex(&vertex))?;
        Ok(Response::new(crate::CreateResponse { created: res }))
    }

    async fn create_vertex_from_type(
        &self,
        request: Request<crate::Identifier>,
    ) -> Result<Response<crate::Uuid>, Status> {
        let t = map_conversion_result(request.into_inner().try_into())?;
        let res = map_indradb_result(self.datastore.create_vertex_from_type(t))?;
        Ok(Response::new(res.into()))
    }

    type GetVerticesStream = Pin<Box<dyn Stream<Item = Result<crate::Vertex, Status>> + Send + Sync + 'static>>;
    async fn get_vertices(
        &self,
        request: Request<crate::VertexQuery>,
    ) -> Result<Response<Self::GetVerticesStream>, Status> {
        let datastore = self.datastore.clone();
        let q: indradb::VertexQuery = map_conversion_result(request.into_inner().try_into())?;
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            send(tx, datastore.get_vertices(q)).await;
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn delete_vertices(&self, request: Request<crate::VertexQuery>) -> Result<Response<()>, Status> {
        let q: indradb::VertexQuery = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.delete_vertices(q))?;
        Ok(Response::new(()))
    }

    async fn get_vertex_count(&self, _: Request<()>) -> Result<Response<crate::CountResponse>, Status> {
        let res = map_indradb_result(self.datastore.get_vertex_count())?;
        Ok(Response::new(crate::CountResponse { count: res }))
    }

    async fn create_edge(&self, request: Request<crate::EdgeKey>) -> Result<Response<crate::CreateResponse>, Status> {
        let key = map_conversion_result(request.into_inner().try_into())?;
        let res = map_indradb_result(self.datastore.create_edge(&key))?;
        Ok(Response::new(crate::CreateResponse { created: res }))
    }

    type GetEdgesStream = Pin<Box<dyn Stream<Item = Result<crate::Edge, Status>> + Send + Sync + 'static>>;
    async fn get_edges(&self, request: Request<crate::EdgeQuery>) -> Result<Response<Self::GetEdgesStream>, Status> {
        let datastore = self.datastore.clone();
        let q: indradb::EdgeQuery = map_conversion_result(request.into_inner().try_into())?;
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            send(tx, datastore.get_edges(q)).await;
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn delete_edges(&self, request: Request<crate::EdgeQuery>) -> Result<Response<()>, Status> {
        let q: indradb::EdgeQuery = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.delete_edges(q))?;
        Ok(Response::new(()))
    }

    async fn get_edge_count(
        &self,
        request: Request<crate::GetEdgeCountRequest>,
    ) -> Result<Response<crate::CountResponse>, Status> {
        let (id, t, direction) = map_conversion_result(request.into_inner().try_into())?;
        let res = map_indradb_result(self.datastore.get_edge_count(id, t.as_ref(), direction))?;
        Ok(Response::new(crate::CountResponse { count: res }))
    }

    type GetVertexPropertiesStream =
        Pin<Box<dyn Stream<Item = Result<crate::VertexProperty, Status>> + Send + Sync + 'static>>;
    async fn get_vertex_properties(
        &self,
        request: Request<crate::VertexPropertyQuery>,
    ) -> Result<Response<Self::GetVertexPropertiesStream>, Status> {
        let datastore = self.datastore.clone();
        let q = map_conversion_result(request.into_inner().try_into())?;
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            send(tx, datastore.get_vertex_properties(q)).await;
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    type GetAllVertexPropertiesStream =
        Pin<Box<dyn Stream<Item = Result<crate::VertexProperties, Status>> + Send + Sync + 'static>>;
    async fn get_all_vertex_properties(
        &self,
        request: Request<crate::VertexQuery>,
    ) -> Result<Response<Self::GetAllVertexPropertiesStream>, Status> {
        let datastore = self.datastore.clone();
        let q: indradb::VertexQuery = map_conversion_result(request.into_inner().try_into())?;
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            send(tx, datastore.get_all_vertex_properties(q)).await;
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn set_vertex_properties(
        &self,
        request: Request<crate::SetVertexPropertiesRequest>,
    ) -> Result<Response<()>, Status> {
        let (q, value) = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.set_vertex_properties(q, value))?;
        Ok(Response::new(()))
    }

    async fn delete_vertex_properties(
        &self,
        request: Request<crate::VertexPropertyQuery>,
    ) -> Result<Response<()>, Status> {
        let q = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.delete_vertex_properties(q))?;
        Ok(Response::new(()))
    }

    type GetEdgePropertiesStream =
        Pin<Box<dyn Stream<Item = Result<crate::EdgeProperty, Status>> + Send + Sync + 'static>>;
    async fn get_edge_properties(
        &self,
        request: Request<crate::EdgePropertyQuery>,
    ) -> Result<Response<Self::GetEdgePropertiesStream>, Status> {
        let datastore = self.datastore.clone();
        let q: indradb::EdgePropertyQuery = map_conversion_result(request.into_inner().try_into())?;
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            send(tx, datastore.get_edge_properties(q)).await;
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    type GetAllEdgePropertiesStream =
        Pin<Box<dyn Stream<Item = Result<crate::EdgeProperties, Status>> + Send + Sync + 'static>>;
    async fn get_all_edge_properties(
        &self,
        request: Request<crate::EdgeQuery>,
    ) -> Result<Response<Self::GetAllEdgePropertiesStream>, Status> {
        let datastore = self.datastore.clone();
        let q: indradb::EdgeQuery = map_conversion_result(request.into_inner().try_into())?;
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            send(tx, datastore.get_all_edge_properties(q)).await;
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn set_edge_properties(
        &self,
        request: Request<crate::SetEdgePropertiesRequest>,
    ) -> Result<Response<()>, Status> {
        let (q, value) = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.set_edge_properties(q, value))?;
        Ok(Response::new(()))
    }

    async fn delete_edge_properties(&self, request: Request<crate::EdgePropertyQuery>) -> Result<Response<()>, Status> {
        let q = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.delete_edge_properties(q))?;
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
        map_indradb_result(datastore.bulk_insert(items))?;
        Ok(Response::new(()))
    }

    async fn index_property(&self, request: Request<crate::IndexPropertyRequest>) -> Result<Response<()>, Status> {
        let name: indradb::Identifier = map_conversion_result(request.into_inner().try_into())?;
        map_indradb_result(self.datastore.clone().index_property(name))?;
        Ok(Response::new(()))
    }
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
pub async fn run<D>(datastore: Arc<D>, listener: TcpListener) -> Result<(), TonicTransportError>
where
    D: indradb::Datastore + Send + Sync + 'static,
{
    let svc = crate::indra_db_server::IndraDbServer::new(Server { datastore });
    let incoming = TcpListenerStream::new(listener);
    TonicServer::builder()
        .add_service(svc)
        .serve_with_incoming(incoming)
        .await?;
    Ok(())
}
