pub mod computer;
pub mod config;
pub mod parser;

use crate::models::ledger::Ledger;
use crate::models::raw_tx::RawTransactionsList;
use log::info;
use std::error::Error;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let result: RawTransactionsList = parser::parse_transactions_file(&config.tx_file)?;
    info!("after parsing result is {:?}", result);

    let ledger: Ledger = computer::compute_ledger(result);
    info!("after compute_ledger ledger is {:?}", ledger);

    Ok(())
}
