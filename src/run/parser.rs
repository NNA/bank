use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct TransactionLine {
    #[serde(rename = "type")]
    kind: TransactionKind,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    tx_id: u32,
    amount: String,
}

pub fn parse_transactions_file(
    file: &dyn AsRef<Path>,
) -> Result<Vec<TransactionLine>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file)?;
    let mut vec: Vec<TransactionLine> = Vec::new();

    for result in rdr.deserialize() {
        println!("Reading line {:?}", result);
        let record: TransactionLine = result?;
        vec.push(record);
        // println!("{:#?}", record);
        // match result {
        //     Ok(record) => {
        //         println!("record {:?}", record);
        //         let _ = parse_record(record);
        //     }
        //     Err(err) => {
        //         return Err(From::from(err));
        //     }
        // }
    }

    Ok(vec)
}
