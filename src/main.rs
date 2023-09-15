mod utils;

use colored::*;
use std::env;
use utils::models;



fn main() {
    println!("\n");
    let welcome_txt = String::from("Hello, Welcome to Miu 🐱").green().bold();

    let args: Vec<String> = env::args().collect();


    models::CLI::Cat(String::from("Value"));
    
    println!("{:#?}", args);
    println!("{}", welcome_txt);
}
