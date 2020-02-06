use actix_http::ResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
pub enum ResError {
  #[fail(display = "internal error")]
  InternalError,
  #[fail(display = "bad request")]
  BadClientData,
  #[fail(display = "timeout")]
  Timeout,
}

impl error::ResponseError for ResError {
  fn error_response(&self) -> HttpResponse {
    ResponseBuilder::new(self.status_code())
      .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
      .body(self.to_string())
  }

  fn status_code(&self) -> StatusCode {
    match *self {
      ResError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
      ResError::BadClientData => StatusCode::BAD_REQUEST,
      ResError::Timeout => StatusCode::GATEWAY_TIMEOUT,
    }
  }
}
