mod auth;
mod game;
mod utils;

use actix_web::web;

pub fn api(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api")
      .configure(game::index)
      .configure(auth::index),
  );
}
