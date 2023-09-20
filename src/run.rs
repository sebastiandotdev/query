use crate::config::Config;
use std::fs;

pub fn run(config: Config) {
    if config.query == "echo" {
        println!("{}", config.value);
    }
    if config.query == "ls" {
        list_file(config)
    }
}

fn list_file(config: Config) {
    let content = fs::read_to_string(config.value).expect("No se encontro el archivo");

    println!("{}", content);
}
