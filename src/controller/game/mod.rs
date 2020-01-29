mod search;

use actix_web::web;
use search as searchRoute;

pub fn index(config: &mut web::ServiceConfig) {
  config.service(web::resource("/search").route(web::get().to(searchRoute::get)));
}
