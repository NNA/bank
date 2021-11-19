use std::error::Error;
use std::fs;

pub struct Config<'a> {
    tx_file: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            panic!("not enough arguments");
        }

        Ok(Config { tx_file: &args[1] })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
