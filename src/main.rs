mod base_url;
mod config;
/**
 * @copyrigth (c) 2023-present, All rights reserved.
 * @castrogarciajs Main file for the cli.
 * Learning Rust by Building Real Applications
 * https://rustlanges.github.io/rust-book-es/
 */
mod error;
mod methods;
mod read;

use clap::Parser;
use colored::Colorize;
use read::ReadConfigQuery;
use std::process;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value = "query.json")]
  config: String,

  #[arg(short, long, default_value = "get")]
  method: String,

  #[arg(short, long, default_value = "/")]
  url: String,
}

fn example_rust_rover() {
  let example_rust_rover_string = String::from("");
  let str_rr = String::new();
  println!("{} {}" , example_rust_rover_string, str_rr)
}

#[tokio::main]
async fn main() {
  let args = Args::parse();
  example_rust_rover();
  const COMMAND_CONFIG: &str = "init";
  let methods = methods::Method {
    get: String::from("get"),
    post: String::from("post"),
    put: String::from("put"),
    delete: String::from("delete"),
  };

  let init_config = config::CommandConfig::new(COMMAND_CONFIG);
  let methods_config = methods::Method::new(methods);

  if args.config == "init" {
    init_config.create(&args.config).unwrap_or_else(|err| {
      eprintln!("{}", err);
      process::exit(1);
    });
    println!("{}", "init command".green());
  }

  if args.method == "get" {
    println!("get method");
    methods_config
      .method_get(&args.method, &args.url)
      .await
      .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
      });
  }
  if args.method == "post" {
    println!("{}\n", String::from("post method").italic().green());
    methods_config
      .method_post(&args.method, &args.url, Box::new("data"))
      .await
      .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
      });
  }

  if args.method == "delete" {
    println!("{}\n", String::from("delete method").italic().green());
    methods_config
      .method_delete(&args.method, &args.url, String::from("1"))
      .await
      .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
      });
  }
}
