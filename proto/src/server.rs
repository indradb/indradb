use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use libloading::Library;
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

#[derive(Debug)]
pub enum PluginError {
    Io(Box<io::Error>),
    LibLoading(libloading::Error),
    Transport(TonicTransportError),
    VersionMismatch {
        library_path: PathBuf,
        indradb_version_info: indradb::plugin::VersionInfo,
        library_version_info: indradb::plugin::VersionInfo,
    },
}

impl StdError for PluginError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            PluginError::Io(ref err) => Some(err),
            PluginError::LibLoading(ref err) => Some(err),
            PluginError::Transport(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PluginError::Io(ref err) => write!(f, "i/o error: {}", err),
            PluginError::LibLoading(ref err) => write!(f, "failed to load library: {}", err),
            PluginError::Transport(ref err) => write!(f, "transport error: {}", err),
            PluginError::VersionMismatch {
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

impl From<io::Error> for PluginError {
    fn from(err: io::Error) -> Self {
        PluginError::Io(Box::new(err))
    }
}

impl From<libloading::Error> for PluginError {
    fn from(err: libloading::Error) -> Self {
        PluginError::LibLoading(err)
    }
}

impl From<TonicTransportError> for PluginError {
    fn from(err: TonicTransportError) -> Self {
        PluginError::Transport(err)
    }
}

#[derive(Default)]
struct Plugins {
    entries: HashMap<String, Box<dyn indradb::plugin::Plugin>>,
    // Kept to ensure libraries aren't dropped
    #[allow(dead_code)]
    libraries: Vec<Library>,
}

/// The IndraDB server implementation.
#[derive(Clone)]
pub struct Server<
    D: indradb::Datastore<Trans = T> + Send + Sync + 'static,
    T: indradb::Transaction + Send + Sync + 'static,
> {
    datastore: Arc<D>,
    plugins: Arc<Plugins>,
}

impl<D: indradb::Datastore<Trans = T> + Send + Sync + 'static, T: indradb::Transaction + Send + Sync + 'static>
    Server<D, T>
{
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
    /// * `plugin_path`: Path to the plugins.
    ///
    /// # Errors
    /// This will return an error if the plugin(s) failed to load.
    ///
    /// # Safety
    /// Loading and executing plugins is inherently unsafe. Only run libraries
    /// that you've vetted.
    pub unsafe fn new_with_plugins<P: AsRef<Path>>(datastore: Arc<D>, plugin_path: P) -> Result<Self, PluginError> {
        let mut libraries = Vec::new();
        let mut plugin_entries = HashMap::new();

        let indradb_version_info = indradb::plugin::indradb_version_info();

        for entry in fs::read_dir(plugin_path.as_ref())? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let library = Library::new(plugin_path.as_ref().as_os_str())?;

                let decl = library
                    .get::<*mut indradb::plugin::PluginDeclaration>(b"plugin_declaration\0")?
                    .read();

                if decl.version_info != indradb_version_info {
                    return Err(PluginError::VersionMismatch {
                        library_path: entry.path(),
                        library_version_info: decl.version_info,
                        indradb_version_info,
                    });
                }

                (decl.register)(&mut plugin_entries);
                libraries.push(library);
            }
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
        map_indradb_result(datastore.bulk_insert(items))?;
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
                let trans = map_indradb_result(self.datastore.clone().transaction())?;
                map_indradb_result(plugin.call(Box::new(trans), arg))?
            };
            Ok(Response::new(crate::ExecutePluginResponse {
                value: Some(response.into()),
            }))
        } else {
            Err(Status::not_found("unknown plugin"))
        }
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
/// * `plugin_path`: Path to the plugins.
///
/// # Errors
/// This will return an error if the gRPC fails to start on the given
/// listener.
///
/// # Safety
/// Loading and executing plugins is inherently unsafe. Only run libraries that
/// you've vetted.
pub async unsafe fn run_with_plugins<D, T, P>(
    datastore: Arc<D>,
    listener: TcpListener,
    plugin_path: P,
) -> Result<(), PluginError>
where
    D: indradb::Datastore<Trans = T> + Send + Sync + 'static,
    T: indradb::Transaction + Send + Sync + 'static,
    P: AsRef<Path>,
{
    let server = Server::new_with_plugins(datastore, plugin_path)?;
    let service = crate::indra_db_server::IndraDbServer::new(server);
    let incoming = TcpListenerStream::new(listener);
    TonicServer::builder()
        .add_service(service)
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}
