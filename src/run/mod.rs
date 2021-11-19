pub mod config;
pub mod parser;

use parser::TransactionLine;
use std::error::Error;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let result: Result<Vec<TransactionLine>, Box<dyn Error>> =
        parser::parse_transactions_file(&config.tx_file);

    println!("result ! {:#?}", result);

    Ok(())
}
