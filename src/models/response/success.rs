use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponseBody {
  pub message: String,
}

impl Default for SuccessResponseBody {
  fn default() -> Self {
    SuccessResponseBody {
      message: "success".to_string(),
    }
  }
}
