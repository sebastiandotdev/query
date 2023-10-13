/**
 * COPYRIGTH CLIPPY INICIALIZATION CLI
 * Clipppy --Software Open Sources
 * Manteiner @castrogarciajs
 *  
*/
mod args;
mod config;
mod methods;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  method: String,

  #[arg(short, long)]
  url: String,

  #[arg(short, long, required = true)]
  config: String,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  config::config(args.config);
  args::run(&args.method, &args.url).await;
}
