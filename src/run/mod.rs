pub mod config;
pub mod parser;
pub mod tx;

use std::error::Error;
use tx::TransactionLine;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let _result: Vec<TransactionLine> = parser::parse_transactions_file(&config.tx_file)?;

    Ok(())
}
