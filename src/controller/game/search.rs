use crate::controller::utils::response::error;
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub fn index(config: &mut web::ServiceConfig) {
  config.service(web::scope("/search").service(web::resource("").route(web::get().to(get))));
}

#[derive(Deserialize)]
pub struct Search {
  name: Option<String>,
  page: Option<i32>,
  per_page: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Game {
  id: String,
  name: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
  games: Vec<Game>,
}

struct Params {
  order_by: String,
  ascending: bool,
  client_id: String,
  name: String,
  limit: i32,
  skip: i32,
}

impl Params {
  pub fn new(client_id: String, name: String, limit: i32, skip: i32) -> Params {
    Params {
      order_by: String::from("popularity"),
      ascending: false,
      client_id: client_id,
      name: name,
      skip: skip,
      limit: limit,
    }
  }
  pub fn to_param(self) -> Vec<(&'static str, String)> {
    vec![
      ("order_by", self.order_by),
      ("ascending", self.ascending.to_string()),
      ("client_id", self.client_id),
      ("name", self.name),
      ("limit", self.limit.to_string()),
      ("skip", self.skip.to_string()),
    ]
  }
}

async fn get(search: web::Query<Search>) -> Result<impl Responder, error::ResError> {
  let client = Client::new();
  let api_key = env::var("BOARDGAME_API_KEY").unwrap();
  let limit = if let Some(num) = search.per_page {
    num
  } else {
    10
  };

  let skip = if let Some(num) = search.page {
    limit * (num - 1)
  } else {
    0
  };

  match &search.name {
    Some(name) => {
      let resp = client
        .get("https://www.boardgameatlas.com/api/search")
        .query(&Params::new(api_key, name.to_string(), limit, skip).to_param())
        .send()
        .await
        .unwrap();
      let data = resp.json::<Response>().await.unwrap();
      Ok(HttpResponse::Ok().json(data))
    }
    None => Err(error::ResError::BadClientData),
  }
}
