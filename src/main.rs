mod config;
mod run;


use colored::*;
use std::env;
use std::process;
use config::Config;
use run::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err.red().bold());
        process::exit(1);
    });

    run(config);  
}
