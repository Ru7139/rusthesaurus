use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[actix_web::test]
async fn the_server() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let client = reqwest::Client::new();

    tokio::spawn(async move {
        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .route("heypage", web::get().to(heypage))
        })
        .listen(listener)
        // .bind(("127.0.0.1", addr.port()))
        .unwrap()
        .run()
        .await
    });

    assert_eq!(
        client
            .get(format!("http://{}/", addr))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
        "hello"
    );

    assert_eq!(
        client
            .post(format!("http://{}/echo", addr))
            .body("can you do echo".to_string())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
        "can you do echo"
    );

    assert_eq!(
        client
            .get(format!("http://{}/heypage", addr))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
        "Heypage"
    );
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn heypage() -> impl Responder {
    HttpResponse::Ok().body("Heypage")
}
