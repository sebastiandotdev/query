pub struct Config {
    pub query: String,
    pub value: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            let msg = "Error: I need success a query";
            Config::help();
            return Err(msg);
        }

        let query = args[1].clone();
        let value = args[2].clone();

        Ok(Config { query, value })
    }

    pub fn help() -> &'static str {
        let msg_help = "
Usage:
        
echo   Print message console
ls     File List
find   Check if a file exists
read   Read the content of a file
        
";
        println!("{}", msg_help);

        msg_help
    }
}
