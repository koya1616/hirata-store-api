#[macro_use]
extern crate rocket;

pub mod models;
pub mod runtime_config;
pub mod supabase;

use rocket::data::{Limits, ToByteUnit};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use crate::models::response::error::{create_error, ErrorResponse};
use crate::models::response::success::SuccessResponseBody;
use crate::models::response::upload::UploadIpaResponseBody;
use crate::supabase::storage::file::temp_file_to_vec;
use crate::supabase::storage::SupabaseStorage;

#[post("/upload", data = "<upload>")]
async fn upload(mut upload: Form<Upload<'_>>) -> Result<Json<UploadIpaResponseBody>, Custom<Json<ErrorResponse>>> {
  let buffer = match temp_file_to_vec(&mut upload.file).await {
    Ok(buffer) => buffer,
    Err(_) => return Err(create_error(Status::BadRequest, "ファイルの読み込みに失敗しました。")),
  };

  let storage = SupabaseStorage::new("ipa".to_string(), upload.name.clone());
  match storage.upload(buffer).await {
    Ok(response) => match response.status().is_success() {
      true => Ok(Json(UploadIpaResponseBody {
        name: upload.name.clone(),
      })),
      false => Err(create_error(
        Status::InternalServerError,
        &response.text().await.unwrap(),
      )),
    },
    Err(_) => Err(create_error(
      Status::InternalServerError,
      "ファイルのアップロードに失敗しました。",
    )),
  }
}

#[get("/")]
fn index() -> Json<SuccessResponseBody> {
  Json(SuccessResponseBody::default())
}

#[catch(401)]
fn unauthorized_catcher(_status: Status, _request: &Request) -> Json<ErrorResponse> {
  Json(ErrorResponse {
    message: "認証トークンが無効または期限切れです。再度ログインしてください。".to_string(),
  })
}

#[launch]
fn rocket() -> _ {
  let limits = Limits::new()
    .limit("form", 100.mebibytes())
    .limit("data-form", 100.mebibytes())
    .limit("file", 50.mebibytes());

  let figment = rocket::Config::figment()
    .merge(("address", "0.0.0.0"))
    .merge(("limits", limits));

  rocket::custom(figment)
    .mount("/", routes![index, upload])
    .register("/", catchers![unauthorized_catcher])
}
