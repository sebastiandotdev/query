use colored::*;
use std::env;
use std::process;

struct Config {
    query: String,
    value: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            let msg = "Error: I need success a query";
            return Err(msg);
        }

        let query = args[1].clone();
        let value = args[2].clone();

        Ok(Config { query, value })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err.red().bold());
        process::exit(1);
    });

    run(config);
    
}


fn run(config: Config) {

    if config.query == "echo" {
        println!("{}", config.value);
    }
}