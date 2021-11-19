use crate::run::tx::TransactionLine;
use csv::Reader;
use std::error::Error;
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::tx::TransactionKind::*;
    use crate::run::tx::TransactionLine;

    #[test]
    fn parser_works_if_given_a_correct_file() {
        let ok_res: Vec<TransactionLine> =
            parse_transactions_file(&"tests/fixtures/regular_tx.csv").unwrap();

        assert_eq!(ok_res.len(), 5);

        assert_eq!(
            ok_res[0],
            TransactionLine {
                kind: Deposit,
                client_id: 1,
                tx_id: 1,
                amount: "1.0".to_string(),
            }
        );

        assert_eq!(
            ok_res[1],
            TransactionLine {
                kind: Deposit,
                client_id: 2,
                tx_id: 2,
                amount: "2.0".to_string(),
            }
        );

        assert_eq!(
            ok_res[2],
            TransactionLine {
                kind: Deposit,
                client_id: 1,
                tx_id: 3,
                amount: "2.0".to_string(),
            }
        );

        assert_eq!(
            ok_res[3],
            TransactionLine {
                kind: Withdrawal,
                client_id: 1,
                tx_id: 4,
                amount: "1.5".to_string(),
            }
        );

        assert_eq!(
            ok_res[4],
            TransactionLine {
                kind: Withdrawal,
                client_id: 2,
                tx_id: 5,
                amount: "3.0".to_string(),
            }
        );
    }
}
