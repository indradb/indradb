extern crate clap;

mod cli;

use std::error::Error;
use std::ffi::OsString;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use crate::cli::CliDatastoreArgs;

use cfg_if::cfg_if;
use indradb_proto as proto;
use tokio::net::TcpListener;

#[cfg(unix)]
fn read_memory_datastore(path: OsString, every: u16) -> Result<Arc<indradb::MemoryDatastore>, Box<dyn Error>> {
    use std::fs::File;
    use std::io::Error as IoError;
    use std::path::Path;
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    let datastore = if Path::new(path.as_os_str()).exists() {
        // TODO: wrap file reader in a buffer for substantially better
        // performance
        Arc::new(indradb::MemoryDatastore::read_image(File::open(path.as_os_str())?)?)
    } else {
        Arc::new(indradb::MemoryDatastore::default())
    };

    {
        let datastore = datastore.clone();
        let path = path.clone();

        spawn(move || {
            loop {
                sleep(Duration::from_secs(every.into()));

                unsafe {
                    let pid = libc::fork();
                    if pid < 0 {
                        panic!("failed to fork: {}", IoError::last_os_error());
                    } else if pid == 0 {
                        // this is the child process
                        // TODO: write to temporary file first, then move it into
                        // the right path
                        // TODO: wrap file writer in a buffer for substantially
                        // better performance
                        let f = File::create(path.as_os_str()).expect(&format!("expected to be able to create file"));
                        datastore
                            .write_image(f)
                            .expect("expected to be able to write the image");
                        std::process::exit(0);
                    } else {
                        // this is the parent process
                        let mut status = 0 as libc::c_int;
                        if libc::waitpid(pid, &mut status, 0) < 0 {
                            panic!("could not wait for child: {}", IoError::last_os_error());
                        } else if status != 0 {
                            panic!("sync failed with child return code: {}", status);
                        }
                    }
                }
            }
        });
    }

    Ok(datastore)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse_cli_args();

    let addr = args.addr.to_socket_addrs()?.next().unwrap();
    let listener = TcpListener::bind(addr).await?;
    let binding = listener.local_addr()?;
    println!("grpc://{}", binding);

    match args.datastore_args {
        CliDatastoreArgs::Rocksdb { path, max_open_files } => {
            let datastore = indradb::RocksdbDatastore::new(&path, Some(max_open_files))
                .expect("Expected to be able to create the RocksDB datastore");

            proto::run_server(Arc::new(datastore), listener).await?;
            Ok(())
        }
        CliDatastoreArgs::Sled { path, sled_config } => {
            let datastore = sled_config
                .open(&path)
                .expect("Expected to be able to create the Sled datastore");

            proto::run_server(Arc::new(datastore), listener).await?;
            Ok(())
        }
        CliDatastoreArgs::Memory { path: None, every: _ } => {
            let datastore = indradb::MemoryDatastore::default();
            proto::run_server(Arc::new(datastore), listener).await?;
            Ok(())
        }
        CliDatastoreArgs::Memory {
            path: Some(path),
            every,
        } => {
            cfg_if! {
                if #[cfg(unix)] {
                    let datastore = read_memory_datastore(path, every)?;
                    proto::run_server(datastore.clone(), listener).await?;
                } else {
                    unreachable!("path shouldn't be set on non-unix systems");
                }
            };
            Ok(())
        }
    }
}
