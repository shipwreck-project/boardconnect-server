use actix_web::{web, Error, HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct Search {
  name: String,
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

pub async fn get(search: web::Query<Search>) -> Result<impl Responder, Error> {
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
  let resp = client.get(
    &format!(
      "https://www.boardgameatlas.com/api/search?order_by=popularity&ascending=false&client_id={}&name={}&limit={}&skip={}",
      api_key,
      search.name,
      limit,
      skip,
    )[..],
  ).send().await.unwrap();
  let data = resp.json::<Response>().await.unwrap();
  Ok(HttpResponse::Ok().json(data))
}
