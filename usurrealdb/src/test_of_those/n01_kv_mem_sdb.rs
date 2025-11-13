use axum::routing::post;
use serde::{Deserialize, de::DeserializeOwned};
use surrealdb::{
    Surreal,
    engine::any::{Any, connect},
};

#[derive(Deserialize, Debug)]
struct DBQueryTest {
    rust_touch: String,
}

async fn sdb_query_default<T>(
    sdb: Surreal<Any>,
    query_msg: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    Ok(sdb.query(query_msg).await?.take::<Vec<T>>(0usize)?)
}

#[tokio::test]
async fn in_memory_sdb_test() {
    let sdb = surrealdb::engine::any::connect("memory").await.unwrap();

    sdb.use_ns("test_ns").use_db("test_db").await.unwrap();

    sdb.query("INSERT INTO test_table {rust_touch: \"test can be done\", test_record: 0}")
        .await
        .unwrap();

    assert_eq!(
        sdb_query_default::<DBQueryTest>(
            sdb.clone(),
            "SELECT rust_touch from test_table where test_record = 0"
        )
        .await
        .unwrap()[0]
            .rust_touch,
        "test can be done"
    );
}

static DB2: std::sync::OnceLock<Surreal<Any>> = std::sync::OnceLock::new();

async fn post_query(query_msg: String) -> Result<String, String> {
    let mut res = DB2
        .get()
        .unwrap()
        .query(query_msg)
        .await
        .map_err(|e| e.to_string())?;
    Ok(res
        .take::<surrealdb::Value>(0)
        .map_err(|e| e.to_string())?
        .to_string())
}

#[tokio::test]
async fn in_memory_sdb_tcp_test() {
    DB2.set(connect("memory").await.unwrap()).unwrap();
    DB2.get()
        .unwrap()
        .use_ns("test_ns")
        .use_db("test_db")
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("localhost:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let router = axum::Router::new().route("/query", post(post_query));

    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, router).await {
            eprintln!("server error: {}", e);
        }
    });

    let client = reqwest::Client::new();
    let response_text = client
        .post(format!("http://{}/query", addr))
        .header("Content-Type", "application/json")
        .body(r#"CREATE user SET name = "Billy""#)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // { id: user:oktnnywt6mj9f5oka7d6, name: 'Billy' }]
    assert_eq!(&response_text[0..=5], "[{ id:");

    let response_text = client
        .post(format!("http://{}/query", addr))
        .header("Content-Type", "application/json")
        .body(r#"LET $x: string = 9"#)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(response_text, "Found 9 for param $x, but expected a string");
}
