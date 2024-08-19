use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

use crate::supabase::request::{Headers, HeadersTypes};

impl Default for Headers {
  fn default() -> Self {
    Self::new()
  }
}

impl Headers {
  pub fn new() -> Self {
    dotenv().ok();
    let supabase_key = env::var("SUPABASE_KEY").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
      HeadersTypes::ApiKey.as_str(),
      HeaderValue::from_str(&supabase_key).unwrap(),
    );
    headers.insert(
      HeadersTypes::Authorization.as_str(),
      HeaderValue::from_str(&format!("Bearer {}", supabase_key)).unwrap(),
    );
    Headers { headers }
  }

  pub fn octet_stream(&mut self) -> &mut Self {
    self.headers.insert(
      HeadersTypes::ContentType.as_str(),
      HeaderValue::from_static("application/octet-stream"),
    );
    self
  }

  pub fn into_header_map(self) -> HeaderMap {
    self.headers
  }
}
