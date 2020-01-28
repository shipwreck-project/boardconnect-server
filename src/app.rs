pub mod data;

use crate::services;
use actix_web::middleware;
use env_logger;

pub fn create_logger() -> middleware::Logger {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  middleware::Logger::default()
}

pub fn create_data() -> data::Data {
  data::Data::new(create_pool(), "/")
}

fn create_pool() -> services::Pool {
  use services::init_pool;
  init_pool()
}
