#[tokio::test]
async fn hello_mini_redis() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        mini_redis::server::run(listener, tokio::signal::ctrl_c())
            .await
            .unwrap();
    });

    let mut client = mini_redis::client::connect(&addr.to_string())
        .await
        .unwrap();

    client.set("hello", "world".into()).await.unwrap();

    let result = client.get("hello").await.unwrap();

    assert_eq!("world".as_bytes(), result.unwrap());
}
