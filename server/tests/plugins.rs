use std::error::Error;
use std::process::{Child, Command};

use serde_json::json;
use tokio::time::{sleep, Duration};

#[cfg(target_os = "macos")]
static LIBRARY_EXTENSION: &str = "dylib";
#[cfg(target_os = "linux")]
static LIBRARY_EXTENSION: &str = "so";

struct Server {
    child: Child,
}

impl Server {
    pub fn start(plugins_dest_path: &str) -> Result<Self, Box<dyn Error>> {
        let child = Command::new("../target/debug/indradb-server")
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

#[tokio::test]
pub async fn plugins() {
    let _server = Server::start(&format!("../target/debug/libindradb_plugin_*.{}", LIBRARY_EXTENSION)).unwrap();
    let mut client = get_client_retrying().await.unwrap();

    run_test(
        &mut client,
        "hello_world",
        json!("plugin tester"),
        json!("hello, \"plugin tester\""),
    )
    .await
    .unwrap();

    run_test(&mut client, "naive_vertex_count", json!(null), json!(0))
        .await
        .unwrap();
    run_test(&mut client, "naive_vertex_count", json!({"t_filter": "foo"}), json!(0))
        .await
        .unwrap();
    client
        .bulk_insert(vec![
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
        ])
        .await
        .unwrap();
    run_test(&mut client, "naive_vertex_count", json!(null), json!(11))
        .await
        .unwrap();
    run_test(&mut client, "naive_vertex_count", json!({"t_filter": "foo"}), json!(0))
        .await
        .unwrap();
}
