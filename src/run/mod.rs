pub mod computer;
pub mod config;
pub mod parser;
pub mod tx;

use crate::run::tx::BalanceBook;
use log::info;
use std::error::Error;
use tx::RawTransactionsList;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let result: RawTransactionsList = parser::parse_transactions_file(&config.tx_file)?;
    info!("after parsing result is {:?}", result);

    let balance: BalanceBook = computer::compute_balance(result);
    info!("after compute_balance balance is {:?}", balance);

    Ok(())
}
