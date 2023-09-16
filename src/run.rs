use crate::config::Config;

pub fn run(config: Config) {
    
    if config.query == "echo" {
        println!("{}", config.value);
    }
    
}