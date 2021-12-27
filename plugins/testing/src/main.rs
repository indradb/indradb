use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{Child, Command, Stdio};

use serde_json::json;
use tempdir::TempDir;
use tokio::time::{sleep, Duration};
use tonic::transport::Endpoint;

struct Server {
    child: Child,
    address: String,
}

impl Server {
    pub fn start(plugins_dest_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut child = Command::new("target/debug/indradb-server")
            .args(["--address", "127.0.0.1:0", "-p", plugins_dest_path, "memory"])
            .env("RUST_BACKTRACE", "1")
            .stdout(Stdio::piped())
            .spawn()?;

        let mut lines = io::BufReader::new(child.stdout.take().unwrap()).lines();
        let address = lines.next().unwrap()?;
        Ok(Server { child, address })
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.child.kill().expect("failed to kill server");
    }
}

fn populate_plugins(plugins_dest_path: &Path) -> Result<(), Box<dyn Error>> {
    // get a list of plugins
    let mut plugins = Vec::<String>::new();
    for entry in fs::read_dir("plugins")? {
        let entry = entry?;
        if entry.file_name() != "testing" && entry.file_name() != "host" {
            plugins.push(entry.file_name().into_string().unwrap());
        }
    }

    // copy built assets into a workspace
    for plugin in &plugins {
        for ext in ["so", "dylib"] {
            let filename = format!("libindradb_plugin_{}.{}", plugin, ext);
            let plugin_src_path_str = format!("target/debug/{}", filename);
            let plugin_src_path = Path::new(&plugin_src_path_str);
            if plugin_src_path.exists() {
                let mut plugin_dest_path = plugins_dest_path.to_owned();
                plugin_dest_path.push(filename);
                fs::copy(plugin_src_path, plugin_dest_path)?;
            }
        }
    }

    Ok(())
}

async fn run_test(
    client: &mut indradb_proto::Client,
    name: &str,
    arg: serde_json::Value,
    expected_response: serde_json::Value,
) -> Result<(), Box<dyn Error>> {
    let response = client.execute_plugin(name.to_string(), arg).await?;
    assert_eq!(response, expected_response);
    Ok(())
}

async fn run_all_tests(plugins_path: &str) -> Result<(), Box<dyn Error>> {
    let server = Server::start(plugins_path)?;
    let endpoint: Endpoint = server.address.clone().try_into()?;
    let mut client = indradb_proto::Client::new(endpoint).await?;
    let mut last_err: Option<indradb_proto::ClientError> = None;

    for i in 0..10 {
        sleep(Duration::from_secs(1)).await;
        match client.ping().await {
            Ok(_) => {
                last_err = None;
                break;
            }
            Err(err) => {
                last_err = Some(err);
                eprintln!("waiting for server [{}]", i + 1);
            }
        };
    }
    if let Some(err) = last_err {
        eprintln!("server failed to start after a few seconds");
        return Err(Box::new(err));
    }

    run_test(
        &mut client,
        "hello_world",
        json!("plugin tester"),
        json!("hello, \"plugin tester\""),
    )
    .await?;

    run_test(&mut client, "vertex_count", json!(null), json!(0)).await?;

    run_test(&mut client, "vertex_count", json!({"t_filter": "foo"}), json!(0)).await?;

    Ok(())
}

#[tokio::main]
pub async fn main() {
    let plugins_path = TempDir::new("indradb-plugin-testing").unwrap();
    populate_plugins(plugins_path.path()).unwrap();
    run_all_tests(plugins_path.path().to_str().unwrap()).await.unwrap();
}
