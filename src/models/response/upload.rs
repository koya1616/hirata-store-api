use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IpaResponseData {
  #[serde(rename = "Id")]
  id: String,
  #[serde(rename = "Key")]
  key: String,
}
