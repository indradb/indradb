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
    use std::fs::rename;
    use std::fs::File;
    use std::io::Error as IoError;
    use std::io::{BufReader, BufWriter};
    use std::path::{Path, PathBuf};
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    let datastore = if Path::new(path.as_os_str()).exists() {
        let buf = BufReader::new(File::open(path.as_os_str())?);
        Arc::new(indradb::MemoryDatastore::read_image(buf)?)
    } else {
        Arc::new(indradb::MemoryDatastore::default())
    };

    {
        let datastore = datastore.clone();
        let persist_path = path.clone();
        let scratch_path = {
            let mut scratch_path = PathBuf::new();
            scratch_path.push(&path);
            scratch_path.set_extension(".tmp");
            scratch_path.into_boxed_path()
        };

        spawn(move || {
            loop {
                sleep(Duration::from_secs(every.into()));

                unsafe {
                    let pid = libc::fork();
                    if pid < 0 {
                        panic!("failed to fork: {}", IoError::last_os_error());
                    } else if pid == 0 {
                        // this is the child process
                        let f = File::create(&scratch_path).expect("expected to be able to create scratch file");
                        datastore
                            .write_image(BufWriter::new(f))
                            .expect("expected to be able to write the image");
                        rename(scratch_path, persist_path).expect("expected to be able to rename scratch file");
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
