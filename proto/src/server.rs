use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use libloading::Library;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_stream::wrappers::{ReceiverStream, TcpListenerStream};
use tokio_stream::{Stream, StreamExt};
use tonic::transport::{Error as TonicTransportError, Server as TonicServer};
use tonic::{Request, Response, Status, Streaming};

const CHANNEL_CAPACITY: usize = 100;

macro_rules! send {
    ($tx:expr, $res:expr) => {
        if let Err(err) = $tx.send($res).await {
            eprintln!("could not send message to client: {}", err);
        }
    };
}

async fn send_many<IT, PT>(tx: mpsc::Sender<Result<PT, Status>>, result: Result<Vec<IT>, indradb::Error>)
where
    IT: Into<PT>,
{
    match map_indradb_result(result) {
        Ok(values) => {
            for value in values {
                send!(tx, Ok(value.into()))
            }
        }
        Err(err) => send!(tx, Err(err)),
    }
}

fn map_indradb_result<T>(res: Result<T, indradb::Error>) -> Result<T, Status> {
    res.map_err(|err| Status::internal(format!("{}", err)))
}

fn map_conversion_result<T>(res: Result<T, crate::ConversionError>) -> Result<T, Status> {
    res.map_err(|err| Status::invalid_argument(format!("{}", err)))
}

#[derive(Debug)]
pub enum InitError {
    LibLoading(libloading::Error),
    Transport(TonicTransportError),
    Pattern(glob::PatternError),
    Glob(glob::GlobError),
    VersionMismatch {
        library_path: PathBuf,
        indradb_version_info: indradb_plugin_host::VersionInfo,
        library_version_info: indradb_plugin_host::VersionInfo,
    },
}

impl StdError for InitError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            InitError::LibLoading(ref err) => Some(err),
            InitError::Transport(ref err) => Some(err),
            InitError::Pattern(ref err) => Some(err),
            InitError::Glob(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InitError::LibLoading(ref err) => write!(f, "failed to load library: {}", err),
            InitError::Transport(ref err) => write!(f, "transport error: {}", err),
            InitError::Pattern(ref err) => write!(f, "pattern error: {}", err),
            InitError::Glob(ref err) => write!(f, "glob error: {}", err),
            InitError::VersionMismatch {
                ref library_path,
                ref indradb_version_info,
                ref library_version_info,
            } => {
                write!(
                    f,
                    "version mismatch: library '{}'={}; IndraDB={}",
                    library_path.to_string_lossy(),
                    library_version_info,
                    indradb_version_info
                )
            }
        }
    }
}

impl From<libloading::Error> for InitError {
    fn from(err: libloading::Error) -> Self {
        InitError::LibLoading(err)
    }
}

impl From<TonicTransportError> for InitError {
    fn from(err: TonicTransportError) -> Self {
        InitError::Transport(err)
    }
}

impl From<glob::PatternError> for InitError {
    fn from(err: glob::PatternError) -> Self {
        InitError::Pattern(err)
    }
}

impl From<glob::GlobError> for InitError {
    fn from(err: glob::GlobError) -> Self {
        InitError::Glob(err)
    }
}

#[derive(Default)]
struct Plugins {
    entries: HashMap<String, Box<dyn indradb_plugin_host::Plugin>>,
    // Kept to ensure libraries aren't dropped
    #[allow(dead_code)]
    libraries: Vec<Library>,
}

/// The IndraDB server implementation.
#[derive(Clone)]
pub struct Server<D: indradb::Datastore + Send + Sync + 'static> {
    datastore: Arc<D>,
    plugins: Arc<Plugins>,
}

impl<D: indradb::Datastore + Send + Sync + 'static> Server<D> {
    /// Creates a new server.
    ///
    /// # Arguments
    /// * `datastore`: The underlying datastore to use.
    pub fn new(datastore: Arc<D>) -> Self {
        Self {
            datastore,
            plugins: Arc::new(Plugins::default()),
        }
    }

    /// Creates a new server with plugins enabled.
    ///
    /// # Arguments
    /// * `datastore`: The underlying datastore to use.
    /// * `library_paths`: Paths to libraries to enable.
    ///
    /// # Errors
    /// This will return an error if the plugin(s) failed to load.
    ///
    /// # Safety
    /// Loading and executing plugins is inherently unsafe. Only run libraries
    /// that you've vetted.
    pub unsafe fn new_with_plugins(datastore: Arc<D>, library_paths: Vec<PathBuf>) -> Result<Self, InitError> {
        let mut libraries = Vec::new();
        let mut plugin_entries = HashMap::new();

        let indradb_version_info = indradb_plugin_host::indradb_version_info();

        for library_path in library_paths {
            let library = Library::new(&library_path)?;

            let func: libloading::Symbol<unsafe extern "C" fn() -> indradb_plugin_host::PluginDeclaration> =
                library.get(b"register")?;
            let decl = func();

            if decl.version_info != indradb_version_info {
                return Err(InitError::VersionMismatch {
                    library_path,
                    library_version_info: decl.version_info,
                    indradb_version_info,
                });
            }

            plugin_entries.extend(decl.entries);
            libraries.push(library);
        }

        Ok(Self {
            datastore,
            plugins: Arc::new(Plugins {
                libraries,
                entries: plugin_entries,
            }),
        })
    }
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
            send_many(tx, datastore.get_vertices(q)).await;
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
            send_many(tx, datastore.get_edges(q)).await;
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
            send_many(tx, datastore.get_vertex_properties(q)).await;
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
            send_many(tx, datastore.get_all_vertex_properties(q)).await;
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
            send_many(tx, datastore.get_edge_properties(q)).await;
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
            send_many(tx, datastore.get_all_edge_properties(q)).await;
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

    async fn execute_plugin(
        &self,
        request: Request<crate::ExecutePluginRequest>,
    ) -> Result<Response<crate::ExecutePluginResponse>, Status> {
        let request = request.into_inner();
        let arg = if let Some(arg) = request.arg {
            map_conversion_result(arg.try_into())?
        } else {
            serde_json::Value::Null
        };

        if let Some(plugin) = self.plugins.entries.get(&request.name) {
            let response = {
                plugin
                    .call(self.datastore.clone(), arg)
                    .map_err(|err| Status::internal(format!("{}", err)))?
            };
            Ok(Response::new(crate::ExecutePluginResponse {
                value: Some(response.into()),
            }))
        } else {
            Err(Status::not_found("unknown plugin"))
        }
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
    let service = crate::indra_db_server::IndraDbServer::new(Server::new(datastore));
    let incoming = TcpListenerStream::new(listener);
    TonicServer::builder()
        .add_service(service)
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}

/// Runs the IndraDB server with plugins enabled.
///
/// # Arguments
/// * `datastore`: The underlying datastore to use.
/// * `listener`: The TCP listener to run the gRPC server on.
/// * `plugin_path_pattern`: A [glob](https://docs.rs/glob/0.3.0/glob/) to the
///   plugin paths to be used.
///
/// # Errors
/// This will return an error if the gRPC fails to start on the given
/// listener.
///
/// # Safety
/// Loading and executing plugins is inherently unsafe. Only run libraries that
/// you've vetted.
pub async unsafe fn run_with_plugins<D>(
    datastore: Arc<D>,
    listener: TcpListener,
    plugin_path_pattern: &str,
) -> Result<(), InitError>
where
    D: indradb::Datastore + Send + Sync + 'static,
{
    let mut plugin_paths = Vec::new();
    for entry in glob::glob(plugin_path_pattern)? {
        plugin_paths.push(entry?);
    }

    let server = Server::new_with_plugins(datastore, plugin_paths)?;
    let service = crate::indra_db_server::IndraDbServer::new(server);
    let incoming = TcpListenerStream::new(listener);
    TonicServer::builder()
        .add_service(service)
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}
