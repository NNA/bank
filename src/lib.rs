use std::error::Error;
use std::io::BufRead;

use std::fs::File;
use std::io::BufReader;

pub struct Config<'a> {
    tx_file: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        Ok(Config { tx_file: &args[1] })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.tx_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("Reading transaction {}", line?);
    }

    Ok(())
}
