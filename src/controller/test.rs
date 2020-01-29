use actix_web::{get, web, HttpResponse};

#[get("/test/{username}")]
pub fn get_test_data(path: web::Path<(String,)>) -> HttpResponse {
  HttpResponse::Ok().json(json!({
    "message": "hello",
    "user": path.0,
  }))
}
