use crate::config::Config;
use std::fs;

pub fn run(config: Config) {
    if config.query == "echo" {
        println!("{}", config.value);
    }
    if config.query == "read" {
        list_file(&config);
    }
    if config.query == "ls" {
        read_folder(&config);
    }
}

fn list_file(config: &Config) {
    let content = fs::read_to_string(&config.value).expect("No se encontro el archivo");

    println!("{}", content);
}

fn read_folder(config: &Config) {
    let folders = fs::read_dir(&config.value).expect("No found folder");

    for entry in folders {
        let entry = entry.expect("Error al leer entrada");
        let entry_path = entry.path();

        if entry_path.is_dir() {
            println!("{:?}", entry_path);
        }
    }
}
