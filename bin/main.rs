use reqwest::Client;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    run().await;
}


async fn run() {

  let client = Client::new();
  let result = client.get("https://jsonplaceholder.typicode.com/users").send().await;

  println!("{:#?}", result);
}