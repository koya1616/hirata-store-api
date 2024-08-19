use serde::Serialize;

#[derive(Serialize)]
pub struct UploadIpaResponseBody {
  pub name: String,
}
