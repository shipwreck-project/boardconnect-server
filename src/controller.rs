mod game;
mod utils;

use actix_web::web;
use game::index;

pub fn api(config: &mut web::ServiceConfig) {
  config.service(web::scope("/api").service(web::scope("/game").configure(index)));
}
