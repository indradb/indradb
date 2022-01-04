use std::collections::HashMap;
use std::error::Error;
use std::process::{Child, Command};

use indradb::VertexQueryExt;
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

fn create_vertex(id: u128) -> indradb::Vertex {
    indradb::Vertex::with_id(
        uuid::Uuid::from_u128(id),
        indradb::Identifier::new(id.to_string()).unwrap(),
    )
}

fn create_edge_key(out_id: u128, in_id: u128) -> indradb::EdgeKey {
    indradb::EdgeKey::new(
        uuid::Uuid::from_u128(out_id),
        indradb::Identifier::new("link").unwrap(),
        uuid::Uuid::from_u128(in_id),
    )
}

async fn test_centrality(client: &mut indradb_proto::Client, cache_edges: bool) {
    // initial weights
    // 1: 1.0
    // 2: 1.0
    // 3: 1.0
    // iteration #0 results
    // 1: 1.0
    // 2: 1.5
    // 3: 2.5
    // total delta: 2.5
    // iteration #1 results
    // 1: 1.0
    // 2: 1.5
    // 3: 3.0
    // total delta: 0.5
    // iteration #0 results
    // 1: 1.0
    // 2: 1.5
    // 3: 3.0
    // total delta: 0.0
    let delta = client
        .execute_plugin("centrality", json!({ "cache_edges": cache_edges }))
        .await
        .unwrap()
        .as_f64()
        .unwrap();
    assert!(delta.abs() <= 0.00001);
    let properties = client
        .get_vertex_properties(
            indradb::RangeVertexQuery::new().property(indradb::Identifier::new("centrality").unwrap()),
        )
        .await
        .unwrap();
    let mut properties_map = HashMap::new();
    for prop in properties {
        properties_map.insert(prop.id.as_u128(), prop.value.as_f64().unwrap());
    }
    assert_eq!(properties_map.len(), 3);
    assert!((properties_map.get(&1).unwrap() - 1.0).abs() < 0.00001);
    assert!((properties_map.get(&2).unwrap() - 1.5).abs() < 0.00001);
    assert!((properties_map.get(&3).unwrap() - 3.0).abs() < 0.00001);
}

#[tokio::test]
pub async fn plugins() {
    let _server = Server::start(&format!("../target/debug/libindradb_plugin_*.{}", LIBRARY_EXTENSION)).unwrap();
    let mut client = get_client_retrying().await.unwrap();

    // insert a sample graph that looks like this:
    //    1
    //  ↙   ↘
    // 2  →  3
    client
        .bulk_insert(vec![
            indradb::BulkInsertItem::Vertex(create_vertex(1)),
            indradb::BulkInsertItem::Vertex(create_vertex(2)),
            indradb::BulkInsertItem::Vertex(create_vertex(3)),
            indradb::BulkInsertItem::Edge(create_edge_key(1, 2)),
            indradb::BulkInsertItem::Edge(create_edge_key(1, 3)),
            indradb::BulkInsertItem::Edge(create_edge_key(2, 3)),
        ])
        .await
        .unwrap();

    assert_eq!(
        client
            .execute_plugin("hello_world", json!("plugin tester"))
            .await
            .unwrap(),
        json!("hello, \"plugin tester\"")
    );

    assert_eq!(
        client.execute_plugin("naive_vertex_count", json!(null)).await.unwrap(),
        json!(3)
    );
    assert_eq!(
        client
            .execute_plugin("naive_vertex_count", json!({"t_filter": "foo"}))
            .await
            .unwrap(),
        json!(0)
    );

    test_centrality(&mut client, false).await;
    test_centrality(&mut client, true).await;
}
