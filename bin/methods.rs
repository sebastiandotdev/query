use reqwest::Client;

pub async fn get(client: Client, url: String) {
  let response = client.get(&url).send().await.expect("Error response");

  let data = response.text().await.expect("failed to get response body");
  println!("response: {:#?}", data);
}
