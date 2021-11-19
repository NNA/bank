pub mod config;
pub mod parser;

use parser::TransactionLine;
use std::error::Error;
use std::path::Path;
// use std::fs::File;
// use std::io::BufReader;

use csv::Reader;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let result: Result<Vec<TransactionLine>, Box<dyn Error>> =
        parse_transactions_file(&config.tx_file);

    println!("result ! {:#?}", result);

    Ok(())
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
