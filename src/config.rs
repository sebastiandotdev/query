use std::{fs, process};

use crate::error::CustomError;
use colored::Colorize;

pub struct CommandConfig {
  init: String,
}
impl CommandConfig {
  pub fn new(command: &str) -> Self {
    Self {
      init: String::from(command),
    }
  }

  pub fn create(&self, option: &String) -> Result<(), CustomError> {
    if &self.init != option {
      let err = CustomError::new("Invalid command see the --help option");
      return Err(err);
    }

    let config = serde_json::json!({
      "title": "Welcome to query cli",
      "description": "Query CLI to do request http",
      "version": "0.1.0",
      "license": "MIT",
      "repository": "https://github.com/castrogarciajs/query",
      "keywords": ["cli", "query", "rust"],
      "base_url": "http://localhost:8080",
      "methods": ["GET", "POST", "DELETE"]
    });

    let formatted_json =
      serde_json::to_string_pretty(&config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
      });

    let create_config = fs::write("query.json", formatted_json);

    if create_config.is_err() {
      let err = CustomError::new("Error creating file");
      return Err(err);
    }

    println!("{}", "created file succesfully!".green());
    Ok(())
  }
}
