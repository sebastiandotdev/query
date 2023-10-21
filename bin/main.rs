/**
 * @copyrigth (c) 2023-present, All rights reserved.
 * @castrogarciajs Main file for the cli.
 * Learning Rust by Building Real Applications
 * https://rustlanges.github.io/rust-book-es/
 */
mod error;

use clap::Parser;
use colored::Colorize;
use error::CustomError;
use serde_json::{json, to_string_pretty};
use std::{fs, process};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  config: String,
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

    let config = json!({
      "title": "Welcome to fetchy cli",
      "description": "This is a cli for fetchy",
      "version": "0.1.0",
      "license": "MIT",
      "repository": "https://github.com/castrogarciajs/rusty_fetchy",
      "keywords": ["cli", "fetchy", "rust"],
      "base_url": "https://{to_url}",
      "methods": ["GET", "POST", "PUT", "DELETE"]
    });

    let formatted_json = to_string_pretty(&config).unwrap_or_else(|err| {
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

fn main() {
  let args = Args::parse();
  const COMMAND_CONFIG: &str = "init";
  let init_config = CommandConfig::new(COMMAND_CONFIG);

  let create_config: Result<(), CustomError> = init_config.create(&args.config);

  if let Err(err) = create_config {
    eprintln!("{}", err);
    process::exit(1);
  }
}
