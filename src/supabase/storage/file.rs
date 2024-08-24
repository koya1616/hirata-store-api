use reqwest::{Client, Error, Response};
use rocket::fs::TempFile;

use crate::rocket::tokio::io::AsyncReadExt;
use crate::runtime_config::RuntimeConfig;
use crate::supabase::request::Headers;
use crate::supabase::storage::SupabaseStorage;

impl SupabaseStorage {
  pub fn new(bucket_name: String, filename: String) -> Self {
    SupabaseStorage {
      supabase_url: RuntimeConfig::global().supabase_url.clone(),
      bucket_name,
      filename,
    }
  }

  /// Uploads a file to an existing bucket.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use supabase_rs::SupabaseStorage;
  ///
  /// let storage = SupabaseStorage {
  ///     supabase_url: "https://example.com".to_string(),
  ///     bucket_name: "bucket".to_string(),
  ///     filename: "file.txt".to_string(),
  /// };
  ///
  /// let bytes = storage.upload().await.unwrap();
  /// ```
  pub async fn upload(&self, buffer: Vec<u8>) -> Result<Response, Error> {
    let url: String = format!(
      "{}/storage/v1/object/{}/{}",
      self.supabase_url, self.bucket_name, self.filename
    );

    let mut headers = Headers::new();
    headers.octet_stream();

    let response: Response = Client::new()
      .post(&url)
      .headers(headers.into_header_map())
      .body(buffer)
      .send()
      .await?;

    Ok(response)
  }
}

pub async fn temp_file_to_vec(file: &mut TempFile<'_>) -> Result<Vec<u8>, std::io::Error> {
  let mut buffer = Vec::new();
  let mut opened_file = file.open().await?;
  opened_file.read_to_end(&mut buffer).await?;
  Ok(buffer)
}
