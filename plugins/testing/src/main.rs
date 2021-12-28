use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Child, Command};

use serde_json::json;
use tempdir::TempDir;
use tokio::time::{sleep, Duration};

struct Server {
    child: Child
}

impl Server {
    pub fn start(plugins_dest_path: &str) -> Result<Self, Box<dyn Error>> {
        let child = Command::new("target/debug/indradb-server")
            .args(["-p", plugins_dest_path, "memory"])
            .env("RUST_BACKTRACE", "1")
            .spawn()?;

        Ok(Server { child })
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

async fn get_client() -> Result<indradb_proto::Client, indradb_proto::ClientError> {
    let mut client = indradb_proto::Client::new("grpc://127.0.0.1:27615".try_into().unwrap()).await?;
    client.ping().await?;
    Ok(client)
}

async fn get_client_retrying() -> Result<indradb_proto::Client, indradb_proto::ClientError> {
    let mut retry_count = 10u8;
    let mut last_err = Option::<indradb_proto::ClientError>::None;

    while retry_count > 0 {
        match get_client().await {
            Ok(client) => return Ok(client),
            Err(err) => {
                last_err = Some(err);
                if retry_count == 0 {
                    break;
                } else {
                    sleep(Duration::from_secs(1)).await;
                    retry_count -= 1;
                }
            }
        }
    }

    Err(last_err.unwrap())
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
    let _server = Server::start(plugins_path)?;
    let mut client = get_client_retrying().await?;

    run_test(
        &mut client,
        "hello_world",
        json!("plugin tester"),
        json!("hello, \"plugin tester\""),
    )
    .await?;

    run_test(&mut client, "vertex_count", json!(null), json!(0)).await?;
    run_test(&mut client, "vertex_count", json!({"t_filter": "foo"}), json!(0)).await?;
    client.bulk_insert(vec![
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("1").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("2").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("3").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("4").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("5").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("6").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("7").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("8").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("9").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("10").unwrap())),
        indradb::BulkInsertItem::Vertex(indradb::Vertex::new(indradb::Identifier::new("11").unwrap())),
    ]).await?;
    run_test(&mut client, "vertex_count", json!(null), json!(11)).await?;
    run_test(&mut client, "vertex_count", json!({"t_filter": "foo"}), json!(0)).await?;

    Ok(())
}

#[tokio::main]
pub async fn main() {
    let plugins_path = TempDir::new("indradb-plugin-testing").unwrap();
    populate_plugins(plugins_path.path()).unwrap();
    run_all_tests(plugins_path.path().to_str().unwrap()).await.unwrap();
}
