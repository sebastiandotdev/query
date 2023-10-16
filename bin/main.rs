use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  config: String,
}

fn main() {
  let args = Args::parse();

  let init_config = CommandConfig::new();

  match args.config.as_str() {
    "init" => init_config.create(&args.config),
    _ => println!("No command found"),
  }
}

pub struct CommandConfig {
  init: String,
}

impl CommandConfig {
  fn new() -> CommandConfig {
    const COMMAND_CONFIG: &str = "init";
    CommandConfig {
      init: String::from(COMMAND_CONFIG),
    }
  }

  fn create(&self, option: &String) {
    if option == &self.init {
      println!("Init command");
    }
  }
}
