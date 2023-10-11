use clap::Parser;
pub mod args;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
  #[arg(short, long)]
  method: String,

  #[arg(short, long)]
  url: String,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  args::run(&args.method, &args.url).await;
}