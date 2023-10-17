/**
 * @copyrigth (c) 2023-present, All rights reserved.
 * @castrogarciajs Main file for the cli.
 * Learning Rust by Building Real Applications
 * https://rustlanges.github.io/rust-book-es/
 */
mod error;

use clap::Parser;
use error::CustomError;
use std::process;

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
    let err = CustomError::new("Invalid command see the --help option");

    if &self.init != option {
      return Err(err);
    }

    println!("create file succesfuly");
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
