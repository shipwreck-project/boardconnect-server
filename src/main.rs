extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;

mod app;
mod controller;
mod models;
mod schema;
mod services;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .data(app::create_data())
            .service(controller::index)
            .wrap(app::create_logger())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
