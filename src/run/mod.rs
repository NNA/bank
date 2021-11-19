pub mod config;
pub mod parser;
pub mod tx;

use std::error::Error;
use tx::TransactionLine;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let result: Result<Vec<TransactionLine>, Box<dyn Error>> =
        parser::parse_transactions_file(&config.tx_file);

    println!("result ! {:#?}", result);

    Ok(())
}
