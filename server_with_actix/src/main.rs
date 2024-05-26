use std::time::Duration;

use actix_web::{App, get, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, Actix!"
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().service(index)
    })
        .keep_alive(Duration::from_secs(75))
        .bind("127.0.0.1:8080")?
        .run()
        .await
        .unwrap_or_else(|err| {
            eprintln!("Server error: {}", err);
            std::process::exit(1);
        })
}
