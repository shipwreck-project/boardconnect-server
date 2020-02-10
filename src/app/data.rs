use crate::services::Pool;
use actix_web::web;

pub struct Data {
  pub pool: Pool,
}

impl Data {
  pub fn new(pool: Pool) -> Data {
    Data { pool: pool }
  }
}

pub type GlobalData = web::Data<Data>;
