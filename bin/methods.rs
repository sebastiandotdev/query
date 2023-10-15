/**
 * Functions for method
 * Minify -- Software Open Sources
 * Manteiner @castrogarciajs
 *
 */
use reqwest::Client;

pub struct Methods;

impl Methods {
  pub fn new() -> Methods {
    Methods
  }

  pub async fn get(&self, client: Client, url: String) {
    let response = client.get(&url).send().await.expect("Error response");

    let data = response.text().await.expect("failed to get response body");
    println!("response: {:#?}", data);
  }
}
