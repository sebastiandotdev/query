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
use std::{any::Any, fs, process};

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
      "base_url": "http://{to_url}",
      "methods": ["GET"] // Method supported
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
  async fn method_get(
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

    let res = client
      .get(format!("{}/{}", json.base_url, endpoint))
      .send()
      .await
      .unwrap_or_else(|open_err| {
        eprintln!("{}", open_err.to_string().red());
        process::exit(1);
      });

    let mut content = String::new();
    if let Ok(value) = res.text().await {
      content = value;
    }
    let parsed_json: serde_json::Value = serde_json::from_str(&content)
      .unwrap_or_else(|err| {
        eprintln!("{}", err.to_string().red());
        process::exit(1);
      });
    match parsed_json {
      serde_json::Value::Array(items) => {
        for item in items {
          let json =
            serde_json::to_string_pretty(&item).unwrap_or_else(|err| {
              eprintln!("{}", err.to_string().red());
              process::exit(1);
            });
          println!("{}", json.to_string().italic().green());
        }
      }
      _ => {
        println!("La respuesta no es un array")
      }
    }

    Ok(())
  }
  async fn method_post(
    &self,
    option: &String,
    endpoint: &String,
    _data: Box<dyn Any>,
  ) -> Result<(), CustomError> {
    if &self.post != option {
      let err = CustomError::new("Invalid command see the --help option");
      return Err(err);
    }
    let json_config = ReadConfigFetchy::new().unwrap();
    let json: ConfigJSON = serde_json::from_str(&json_config.json).unwrap();
    let client = reqwest::Client::new();
    let res = client
      .post(format!("{}/{}", json.base_url, endpoint))
      .body("data")
      .send()
      .await
      .unwrap_or_else(|open_err| {
        eprintln!("{}", open_err.to_string().red());
        process::exit(1);
      });
    if res.status().is_success() {
      println!("{}", "created succesfully".green());
    } else {
      println!("{}", "error creating".red());
    }
    Ok(())
  }
}

#[tokio::main]
async fn main() {
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
      .await
      .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
      });
  }
  if args.method == "post" {
    println!("post method");
    methods_config
      .method_post(&args.method, &args.url, Box::new("data"))
      .await
      .unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
      });
  }
}
