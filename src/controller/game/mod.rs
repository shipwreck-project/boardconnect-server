mod search;

use actix_web::web;
use search as searchRoute;

pub fn index(config: &mut web::ServiceConfig) {
  config.service(web::scope("/game").configure(searchRoute::index));
}
