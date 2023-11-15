use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigJSON {
  pub base_url: String,
}
