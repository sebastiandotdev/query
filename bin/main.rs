/**
 * COPYRIGTH CLIPPY INICIALIZATION CLI
 * Minify -- Software Open Sources
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
  #[arg(short, long, default_value = "")]
  method: String,

  #[arg(short, long, default_value = "")]
  url: String,

  #[arg(short, long, default_value = "minify.json")]
  config: String,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  config::config(args.config);
  args::run(&args.method, &args.url).await;
}
