use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  message: String,
}

impl ErrorResponse {
  fn new(message: &str) -> Json<Self> {
    Json(Self {
      message: message.to_string(),
    })
  }
}

pub fn create_error(status: Status, message: &str) -> Custom<Json<ErrorResponse>> {
  Custom(status, ErrorResponse::new(message))
}
