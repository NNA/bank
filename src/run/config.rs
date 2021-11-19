#[derive(Debug)]
pub struct Config<'a> {
    pub tx_file: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        Ok(Config { tx_file: &args[1] })
    }
}
