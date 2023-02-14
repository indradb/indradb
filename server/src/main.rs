extern crate clap;

mod cli;

use std::error::Error;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::sync::Arc;

use crate::cli::CliDatastoreArgs;

use indradb_proto as proto;
use tokio::net::TcpListener;

async fn run_server<D>(
    datastore: indradb::Database<D>,
    listener: TcpListener,
    plugin_path: &Option<String>,
) -> Result<(), Box<dyn Error>>
where
    D: indradb::Datastore + Send + Sync + 'static,
{
    let binding = listener.local_addr()?;
    println!("grpc://{binding}");

    if let Some(plugin_path) = plugin_path {
        unsafe {
            proto::run_server_with_plugins(Arc::new(datastore), listener, plugin_path).await?;
        }
    } else {
        proto::run_server(Arc::new(datastore), listener).await?;
    }

    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse_cli_args();

    let addr = args.addr.to_socket_addrs()?.next().unwrap();
    let listener = TcpListener::bind(addr).await?;

    match args.datastore_args {
        CliDatastoreArgs::Rocksdb {
            path,
            max_open_files,
            repair,
        } => {
            if repair {
                indradb::RocksdbDatastore::repair(&path, &indradb::RocksdbDatastore::get_options(Some(max_open_files)))
                    .expect("Expected to be able to repair the RocksDB datastore");
                println!("repair successful");
                return Ok(());
            }

            let datastore = indradb::RocksdbDatastore::new_db_with_options(
                &path,
                &indradb::RocksdbDatastore::get_options(Some(max_open_files)),
            )
            .expect("Expected to be able to create the RocksDB datastore");
            run_server(datastore, listener, &args.plugin_path).await
        }
        CliDatastoreArgs::Memory { path } => {
            let datastore = match path {
                None => indradb::MemoryDatastore::new_db(),
                Some(path) if Path::new(path.as_os_str()).exists() => indradb::MemoryDatastore::read_msgpack_db(path)?,
                Some(path) => indradb::MemoryDatastore::create_msgpack_db(path),
            };
            run_server(datastore, listener, &args.plugin_path).await
        }
    }
}
