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

use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    app::start().await
}
