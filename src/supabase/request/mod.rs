pub mod headers;

use reqwest::header::HeaderMap;

pub struct Headers {
  pub headers: HeaderMap,
}

pub enum HeadersTypes {
  ApiKey,
  Authorization,
  ContentType,
}

impl HeadersTypes {
  pub fn as_str(&self) -> &str {
    match self {
      HeadersTypes::ApiKey => "apikey",
      HeadersTypes::Authorization => "Authorization",
      HeadersTypes::ContentType => "Content-Type",
    }
  }
}
