use std::convert::TryInto;
use std::error::Error as StdError;
use std::fmt;

use crate::ConversionError;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::transport::{Channel, Endpoint, Error as TonicTransportError};
use tonic::{Request, Status};
use uuid::Uuid;

const CHANNEL_CAPACITY: usize = 100;

/// The error returned if a client operation failed.
#[derive(Debug)]
pub enum ClientError {
    /// Conversion between an IndraDB and its protobuf equivalent failed.
    Conversion { inner: ConversionError },
    /// A gRPC error.
    Grpc { inner: Status },
    /// A transport error.
    Transport { inner: TonicTransportError },
    /// The gRPC channel has been closed.
    ChannelClosed,
}

impl StdError for ClientError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            ClientError::Conversion { ref inner } => Some(inner),
            ClientError::Grpc { ref inner } => Some(inner),
            ClientError::Transport { ref inner } => Some(inner),
            _ => None,
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::Conversion { ref inner } => inner.fmt(f),
            ClientError::Grpc { ref inner } => write!(f, "grpc error: {}", inner),
            ClientError::Transport { ref inner } => write!(f, "transport error: {}", inner),
            ClientError::ChannelClosed => write!(f, "failed to send request: channel closed"),
        }
    }
}

impl From<ConversionError> for ClientError {
    fn from(err: ConversionError) -> Self {
        ClientError::Conversion { inner: err }
    }
}

impl From<Status> for ClientError {
    fn from(err: Status) -> Self {
        ClientError::Grpc { inner: err }
    }
}

impl From<TonicTransportError> for ClientError {
    fn from(err: TonicTransportError) -> Self {
        ClientError::Transport { inner: err }
    }
}

impl<T> From<mpsc::error::SendError<T>> for ClientError {
    fn from(_: mpsc::error::SendError<T>) -> Self {
        ClientError::ChannelClosed
    }
}

/// A higher-level client implementation.
///
/// This should be better suited than the low-level client auto-generated by
/// gRPC/tonic in virtually every case, unless you want to avoid the cost of
/// translating between protobuf types and their IndraDB equivalents. The
/// interface is designed to resemble the datastore and transaction traits in
/// IndraDB, but they cannot implement them directly since the functions here
/// are async.
#[derive(Clone)]
pub struct Client(crate::ProtoClient<Channel>);

impl Client {
    /// Creates a new client.
    ///
    /// # Arguments
    /// * `endpoint`: The server endpoint.
    pub async fn new(endpoint: Endpoint) -> Result<Self, ClientError> {
        let client = crate::ProtoClient::connect(endpoint).await?;
        Ok(Client { 0: client })
    }

    /// Pings the server.
    pub async fn ping(&mut self) -> Result<(), ClientError> {
        self.0.ping(()).await?;
        Ok(())
    }

    /// Syncs persisted content. Depending on the datastore implementation,
    /// this has different meanings - including potentially being a no-op.
    pub async fn sync(&mut self) -> Result<(), ClientError> {
        self.0.sync(()).await?;
        Ok(())
    }

    /// Creates a new vertex. Returns whether the vertex was successfully
    /// created - if this is false, it's because a vertex with the same UUID
    /// already exists.
    ///
    /// # Arguments
    /// * `vertex`: The vertex to create.
    pub async fn create_vertex(&mut self, vertex: &indradb::Vertex) -> Result<bool, ClientError> {
        let vertex: crate::Vertex = vertex.clone().into();
        let res = self.0.create_vertex(vertex).await?;
        Ok(res.into_inner().created)
    }

    /// Creates a new vertex with just a type specification. As opposed to
    /// `create_vertex`, this is used when you do not want to manually specify
    /// the vertex's UUID. Returns the new vertex's UUID.
    ///
    /// # Arguments
    /// * `t`: The type of the vertex to create.
    pub async fn create_vertex_from_type(&mut self, t: indradb::Identifier) -> Result<Uuid, ClientError> {
        let t: crate::Identifier = t.into();
        let res = self.0.create_vertex_from_type(t).await?;
        Ok(res.into_inner().try_into()?)
    }

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn get_vertices(&mut self, q: indradb::VertexQuery) -> Result<Vec<indradb::Vertex>, ClientError> {
        let q: crate::VertexQuery = q.into();
        let mut vertices = Vec::<indradb::Vertex>::new();
        let mut res = self.0.get_vertices(q).await?.into_inner();

        while let Some(res) = res.next().await {
            vertices.push(res?.try_into()?);
        }

        Ok(vertices)
    }

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn delete_vertices(&mut self, q: indradb::VertexQuery) -> Result<(), ClientError> {
        let q: crate::VertexQuery = q.into();
        self.0.delete_vertices(q).await?;
        Ok(())
    }

    /// Gets the number of vertices in the datastore.
    pub async fn get_vertex_count(&mut self) -> Result<u64, ClientError> {
        let res = self.0.get_vertex_count(()).await?;
        Ok(res.into_inner().count)
    }

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `key`: The edge to create.
    pub async fn create_edge(&mut self, key: &indradb::EdgeKey) -> Result<bool, ClientError> {
        let key: crate::EdgeKey = key.clone().into();
        let res = self.0.create_edge(key).await?;
        Ok(res.into_inner().created)
    }

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn get_edges(&mut self, q: indradb::EdgeQuery) -> Result<Vec<indradb::Edge>, ClientError> {
        let q: crate::EdgeQuery = q.into();
        let mut edges = Vec::<indradb::Edge>::new();
        let mut res = self.0.get_edges(q).await?.into_inner();

        while let Some(res) = res.next().await {
            edges.push(res?.try_into()?);
        }

        Ok(edges)
    }

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn delete_edges(&mut self, q: indradb::EdgeQuery) -> Result<(), ClientError> {
        let q: crate::EdgeQuery = q.into();
        self.0.delete_edges(q).await?;
        Ok(())
    }

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    pub async fn get_edge_count(
        &mut self,
        id: Uuid,
        t: Option<&indradb::Identifier>,
        direction: indradb::EdgeDirection,
    ) -> Result<u64, ClientError> {
        let req: crate::GetEdgeCountRequest = (id, t.cloned(), direction).into();
        let res = self.0.get_edge_count(req).await?;
        Ok(res.into_inner().count)
    }

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn get_vertex_properties(
        &mut self,
        q: indradb::VertexPropertyQuery,
    ) -> Result<Vec<indradb::VertexProperty>, ClientError> {
        let q: crate::VertexPropertyQuery = q.into();
        let mut vertex_properties = Vec::<indradb::VertexProperty>::new();
        let mut res = self.0.get_vertex_properties(q).await?.into_inner();

        while let Some(res) = res.next().await {
            vertex_properties.push(res?.try_into()?);
        }

        Ok(vertex_properties)
    }

    /// Gets all vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn get_all_vertex_properties(
        &mut self,
        q: indradb::VertexQuery,
    ) -> Result<Vec<indradb::VertexProperties>, ClientError> {
        let q: crate::VertexQuery = q.into();
        let mut vertex_properties = Vec::<indradb::VertexProperties>::new();
        let mut res = self.0.get_all_vertex_properties(q).await?.into_inner();

        while let Some(res) = res.next().await {
            vertex_properties.push(res?.try_into()?);
        }

        Ok(vertex_properties)
    }

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    pub async fn set_vertex_properties(
        &mut self,
        q: indradb::VertexPropertyQuery,
        value: serde_json::Value,
    ) -> Result<(), ClientError> {
        let req: crate::SetVertexPropertiesRequest = (q, value).into();
        self.0.set_vertex_properties(req).await?;
        Ok(())
    }

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn delete_vertex_properties(&mut self, q: indradb::VertexPropertyQuery) -> Result<(), ClientError> {
        let q: crate::VertexPropertyQuery = q.into();
        self.0.delete_vertex_properties(q).await?;
        Ok(())
    }

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn get_edge_properties(
        &mut self,
        q: indradb::EdgePropertyQuery,
    ) -> Result<Vec<indradb::EdgeProperty>, ClientError> {
        let q: crate::EdgePropertyQuery = q.into();
        let mut edge_properties = Vec::<indradb::EdgeProperty>::new();
        let mut res = self.0.get_edge_properties(q).await?.into_inner();

        while let Some(res) = res.next().await {
            edge_properties.push(res?.try_into()?);
        }

        Ok(edge_properties)
    }

    /// Gets all edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn get_all_edge_properties(
        &mut self,
        q: indradb::EdgeQuery,
    ) -> Result<Vec<indradb::EdgeProperties>, ClientError> {
        let q: crate::EdgeQuery = q.into();
        let mut edge_properties = Vec::<indradb::EdgeProperties>::new();
        let mut res = self.0.get_all_edge_properties(q).await?.into_inner();

        while let Some(res) = res.next().await {
            edge_properties.push(res?.try_into()?);
        }

        Ok(edge_properties)
    }

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    pub async fn set_edge_properties(
        &mut self,
        q: indradb::EdgePropertyQuery,
        value: serde_json::Value,
    ) -> Result<(), ClientError> {
        let req: crate::SetEdgePropertiesRequest = (q, value).into();
        self.0.set_edge_properties(req).await?;
        Ok(())
    }

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub async fn delete_edge_properties(&mut self, q: indradb::EdgePropertyQuery) -> Result<(), ClientError> {
        let q: crate::EdgePropertyQuery = q.into();
        self.0.delete_edge_properties(q).await?;
        Ok(())
    }

    /// Bulk inserts many vertices, edges, and/or properties.
    ///
    /// Note that datastores have discretion on how to approach safeguard vs
    /// performance tradeoffs. In particular:
    /// * If the datastore is disk-backed, it may or may not flush before
    ///   returning.
    /// * The datastore might not verify for correctness; e.g., it might not
    ///   ensure that the relevant vertices exist before inserting an edge.
    /// If you want maximum protection, use the equivalent functions in
    /// transactions, which will provide more safeguards.
    ///
    /// # Arguments
    /// * `items`: The items to insert.
    pub async fn bulk_insert(&mut self, items: Vec<indradb::BulkInsertItem>) -> Result<(), ClientError> {
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            for item in items.into_iter() {
                if tx.send(item.into()).await.is_err() {
                    return;
                }
            }
        });

        self.0.bulk_insert(Request::new(ReceiverStream::new(rx))).await?;
        Ok(())
    }

    pub async fn index_property(&mut self, name: indradb::Identifier) -> Result<(), ClientError> {
        let request = Request::new(crate::IndexPropertyRequest {
            name: Some(name.into()),
        });
        self.0.index_property(request).await?;
        Ok(())
    }

    pub async fn execute_plugin<S: Into<String>>(
        &mut self,
        name: S,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, ClientError> {
        let request = Request::new(crate::ExecutePluginRequest {
            name: name.into(),
            arg: Some(arg.into()),
        });
        let response = self.0.execute_plugin(request).await?;
        match response.into_inner().value {
            Some(value) => Ok(value.try_into()?),
            None => Ok(serde_json::Value::Null),
        }
    }
}
