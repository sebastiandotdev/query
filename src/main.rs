mod config;

use colored::*;
use std::env;
use std::process;
use config::Config;


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