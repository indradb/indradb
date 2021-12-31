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

#[tokio::test]
pub async fn plugins() {
    let _server = Server::start(&format!("../target/debug/libindradb_plugin_*.{}", LIBRARY_EXTENSION)).unwrap();
    let mut client = get_client_retrying().await.unwrap();

    assert_eq!(
        client
            .execute_plugin("hello_world", json!("plugin tester"))
            .await
            .unwrap(),
        json!("hello, \"plugin tester\"")
    );

    assert_eq!(
        client.execute_plugin("naive_vertex_count", json!(null)).await.unwrap(),
        json!(0)
    );
    assert_eq!(
        client
            .execute_plugin("naive_vertex_count", json!({"t_filter": "foo"}))
            .await
            .unwrap(),
        json!(0)
    );

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
    assert_eq!(
        client.execute_plugin("naive_vertex_count", json!(null)).await.unwrap(),
        json!(11)
    );
    assert_eq!(
        client
            .execute_plugin("naive_vertex_count", json!({"t_filter": "foo"}))
            .await
            .unwrap(),
        json!(0)
    );
}
