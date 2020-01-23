use crate::services::Pool;
use actix_web::web;

pub struct Data {
  pub pool: Pool,
  pub base_path: String,
}

impl Data {
  pub fn new(pool: Pool, base_path: &str) -> Data {
    Data {
      pool: pool,
      base_path: String::from(base_path),
    }
  }
}

pub type GlobalData = web::Data<Data>;
