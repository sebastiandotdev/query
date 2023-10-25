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
use std::{fs, process};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value = "fetchy.json")]
  config: String,

  #[arg(short, long, default_value = "get")]
  method: String,
}
#[derive(Debug, serde::Deserialize)]
struct ConfigJSON {
  title: String,
  description: String,
  version: String,
  license: String,
  repository: String,
  keywords: Vec<String>,
  base_url: String,
  methods: Vec<String>,
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
  fn method_get(&self) -> serde_json::Result<()> {
    let read_config = ReadConfigFetchy::new().unwrap();

    let json: ConfigJSON = serde_json::from_str(&read_config.json)?;

    println!("{}", json.base_url);
    println!("{:?}", json.methods);
    println!("{}", json.title);
    println!("{}", json.description);
    println!("{}", json.version);
    println!("{}", json.license);
    println!("{}", json.repository);
    println!("{:?}", json.keywords);
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

  let create_config = init_config.create(&args.config);

  let get = methods_config.method_get();

  if let Err(err) = get {
    eprintln!("{}", err);
    process::exit(1);
  }
  if let Err(err) = create_config {
    eprintln!("{}", err);
    process::exit(1);
  }
}
