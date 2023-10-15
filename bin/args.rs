use reqwest::Client;
use serde::Deserialize;
use std::fs::File;

use crate::methods::Methods;

#[derive(Deserialize, Debug)]
struct MinifyConfig {
  url_base: String,
}

pub async fn run(method: &String, url: &String) {
  let file = File::open("minify.config.json").expect("Unable to open file");

  let config: MinifyConfig =
    serde_json::from_reader(file).expect("Unable to read file");

  let client = Client::new();
  let base_url = &config.url_base;

  let url = format!("{}{}", base_url, url);

  actions(method, client, url).await;
}

async fn actions(method: &String, client: Client, url: String) {
  let methods_fn = Methods::new();
  if method == "get" {
    methods_fn.get(client, url).await;
  }
}
