use axum::routing::{get, post};

#[tokio::test]
async fn the_server() {
    let app = axum::Router::new()
        .route("/home", get(homepage))
        .route("/post_home", post(homepage_post));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

    let addr = listener.local_addr().unwrap();
    let client = reqwest::Client::new();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let text1 = client
        .get(format!("http://{}/home", addr))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(text1, "get homepage");

    let text2 = client
        .post(format!("http://{}/post_home", addr))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(text2, "post homepage");
}

async fn homepage() -> &'static str {
    "get homepage"
}

async fn homepage_post() -> &'static str {
    "post homepage"
}
