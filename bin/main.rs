use clap::Parser;
use reqwest::Client;

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
pub struct Args {
  #[arg(short, long)]
  method: String,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();
  println!("Hello, world!");

  run(&args.method).await;
}

async fn run(method: &String) {
  let client = Client::new();

  if method == "get" {
    let reusult = client
      .get("https://jsonplaceholder.typicode.com/users")
      .send()
      .await;

    match reusult {
      Ok(response) => {
        let body = response.text().await.expect("failed to get response body");

        println!("response: {:#?}", body);
      }
      Err(error) => {
        println!("error: {:?}", error);
      }
    }
  }
}
