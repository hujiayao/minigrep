use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}