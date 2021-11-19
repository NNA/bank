use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _config = Config::new(&args);
}

struct Config<'a> {
    transaction_file: &'a str,
}

impl Config<'_> {
    fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("not enough arguments");
        }

        Config {
            transaction_file: &args[1],
        }
    }
}
