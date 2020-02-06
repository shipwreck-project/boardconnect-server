pub mod data;

use crate::controller;
use crate::services;
use actix_web::middleware;
use actix_web::{App, HttpServer};
use env_logger;

pub async fn start() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      // DB를 현재 사용하지 않기 때문에 주석처리
      // .data(create_data())
      .configure(controller::api)
      .wrap(create_logger())
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

pub fn create_logger() -> middleware::Logger {
  middleware::Logger::default()
}

pub fn create_data() -> data::Data {
  data::Data::new(create_pool(), "/")
}

fn create_pool() -> services::Pool {
  use services::init_pool;
  init_pool()
}
