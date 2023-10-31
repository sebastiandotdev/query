/**
 * @copyrigth (c) 2023-present, All rights reserved.
 * @castrogarciajs Main file for the cli.
 * Learning Rust by Building Real Applications
 * https://rustlanges.github.io/rust-book-es/
 */
mod error;
mod read;

use clap::Parser;
use colored::Colorize;
use error::CustomError;
use read::ReadConfigFetchy;
use serde::Deserialize;
use std::{fs, process};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value = "fetchy.json")]
  config: String,

  #[arg(short, long, default_value = "get")]
  method: String,

  #[arg(short, long, default_value = "/")]
  url: String,
}
#[derive(Debug, Deserialize)]
struct ConfigJSON {
  base_url: String,
}
pub struct CommandConfig {
  init: String,
}

impl CommandConfig {
  fn new(command: &str) -> Self {
    Self {
      init: String::from(command),
    }
  }

  fn create(&self, option: &String) -> Result<(), CustomError> {
    if &self.init != option {
      let err = CustomError::new("Invalid command see the --help option");
      return Err(err);
    }

    let config = serde_json::json!({
      "title": "Welcome to fetchy cli",
      "description": "This is a cli for fetchy",
      "version": "0.1.0",
      "license": "MIT",
      "repository": "https://github.com/castrogarciajs/rusty_fetchy",
      "keywords": ["cli", "fetchy", "rust"],
      "base_url": "https://{to_url}",
      "methods": ["GET", "POST", "PUT", "DELETE"]
    });

    let formatted_json =
      serde_json::to_string_pretty(&config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
      });

    let create_config = fs::write("fetchy.json", formatted_json);

    if create_config.is_err() {
      let err = CustomError::new("Error creating file");
      return Err(err);
    }

    println!("{}", "created file succesfully!".green());
    Ok(())
  }
}

struct Method {
  get: String,
  post: String,
  put: String,
  delete: String,
}

impl Method {
  fn new(args: Method) -> Self {
    Self {
      get: args.get,
      post: args.post,
      put: args.put,
      delete: args.delete,
    }
  }
  fn method_get(
    &self,
    option: &String,
    endpoint: &String,
  ) -> Result<(), CustomError> {
    if &self.get != option {
      let err = CustomError::new("Invalid command see the --help option");
      return Err(err);
    }

    let read_config = ReadConfigFetchy::new().unwrap();

    let json: ConfigJSON = serde_json::from_str(&read_config.json).unwrap();

    let client = reqwest::Client::new();

    let _ = client.get(format!("{}{}", json.base_url, endpoint));
    Ok(())
  }
}

fn main() {
  let args = Args::parse();
  const COMMAND_CONFIG: &str = "init";
  let methods = Method {
    get: String::from("get"),
    post: String::from("post"),
    put: String::from("put"),
    delete: String::from("delete"),
  };

  let init_config = CommandConfig::new(COMMAND_CONFIG);
  let methods_config = Method::new(methods);

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
      .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
      });
  }
}
