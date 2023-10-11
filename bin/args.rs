use reqwest::Client;

pub async fn run(method: &String, url: &String) {
  let client = Client::new();
  println!("{}", url);

  if method == "get" {
    let response = client
      .get("https://jsonplaceholder.typicode.com/users")
      .send()
      .await
      .expect("Error response");

    let data = response.text().await.expect("failed to get response body");

    println!("response: {:#?}", data);
  }
}
