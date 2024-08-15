use crate::utils::read_config_file;
use anyhow::Error;
use reqwest::{Client, Method, Response};
use serde_json::Value;

pub async fn http(
  method: Method,
  path: &str,
  body: Option<Value>,
) -> Result<Response, Error> {
  let rc_config = read_config_file()?;
  let base_url = rc_config.url;

  let client = Client::new();
  let url = format!("{}/{}", base_url, path);

  let request_builder = match method {
    Method::GET => client.get(&url),
    Method::POST => {
      if let Some(json_body) = body {
        client.post(&url).json(&json_body)
      } else {
        client.post(&url)
      }
    }
    Method::PUT => {
      if let Some(json_body) = body {
        client.put(&url).json(&json_body)
      } else {
        client.put(&url)
      }
    }
    Method::DELETE => client.delete(&url),
    _ => return Err(Error::msg("Método HTTP no soportado")),
  };

  let response = request_builder.send().await?;
  Ok(response)
}