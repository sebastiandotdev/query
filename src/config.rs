pub struct Config {
    pub query: String,
    pub value: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            let msg = "Error: I need success a query";
            return Err(msg);
        }

        let query = args[1].clone();
        let value = args[2].clone();

        Ok(Config { query, value })
    }
}