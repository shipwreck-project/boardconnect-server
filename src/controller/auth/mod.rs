use actix_web::{web, HttpResponse};

pub fn index(config: &mut web::ServiceConfig) {
  config.service(web::resource("/auth").route(
    web::get().to(|| HttpResponse::Ok().json(json! {{ "message": "you're a good person"}})),
  ));
}
