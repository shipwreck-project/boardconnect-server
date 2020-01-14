extern crate actix_web;

use actix_web::{get, middleware, App, HttpServer, Responder};
use env_logger;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
