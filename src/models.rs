use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Methods {
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Env {
  Local,
  Global,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Headers {
  #[serde(rename = "Authorization")]
  pub authorization: String,

  #[serde(rename = "Content-Type")]
  pub content_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RCConfig {
  pub url: String,
  pub methods: Vec<Methods>,
  pub env: Env,
  pub headers: Headers,
}

impl Methods {
  pub fn from_str(method: &str) -> Option<Self> {
    match method {
      "GET" => Some(Methods::GET),
      "POST" => Some(Methods::POST),
      "PUT" => Some(Methods::PUT),
      "DELETE" => Some(Methods::DELETE),
      "PATCH" => Some(Methods::PATCH),
      _ => None,
    }
  }
}

impl Env {
  pub fn from_str(string: &str) -> Option<Self> {
    match string {
      "Local" => Some(Env::Local),
      "Global" => Some(Env::Global),
      _ => None,
    }
  }
}
