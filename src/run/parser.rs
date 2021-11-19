use crate::run::tx::TransactionLine;
use csv::Reader;
use log::{debug, trace};
use std::error::Error;
use std::path::Path;

pub fn parse_transactions_file(
    file: &dyn AsRef<Path>,
) -> Result<Vec<TransactionLine>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file)?;
    let mut vec: Vec<TransactionLine> = Vec::new();

    for result in rdr.deserialize() {
        debug!("Parsing line {:?}", result);
        // let record: TransactionLine = result?;
        // vec.push(record);
        match result {
            Ok(record) => {
                trace!("record {:?}", record);
                vec.push(record);
                // let _ = parse_record(record);
            }
            Err(err) => {
                return Err(From::from(err));
            }
        }
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
        let res = parse_transactions_file(&"tests/fixtures/regular_tx.csv");

        assert!(res.is_ok());
        let vec: Vec<TransactionLine> = res.unwrap();

        assert_eq!(vec.len(), 5);

        assert_eq!(
            vec[0],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(1),
                tx_id: Some(1),
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            vec[1],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(2),
                tx_id: Some(2),
                amount: Some("2.0".to_string()),
            }
        );

        assert_eq!(
            vec[2],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(1),
                tx_id: Some(3),
                amount: Some("2.0".to_string()),
            }
        );

        assert_eq!(
            vec[3],
            TransactionLine {
                kind: Some(Withdrawal),
                client_id: Some(1),
                tx_id: Some(4),
                amount: Some("1.5".to_string()),
            }
        );

        assert_eq!(
            vec[4],
            TransactionLine {
                kind: Some(Withdrawal),
                client_id: Some(2),
                tx_id: Some(5),
                amount: Some("3.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_unknown_kind() {
        let res = parse_transactions_file(&"tests/fixtures/unknown_kind.csv");

        assert!(res.is_ok());
        let vec: Vec<TransactionLine> = res.unwrap();

        assert_eq!(vec.len(), 2);

        assert_eq!(
            vec[0],
            TransactionLine {
                kind: None,
                client_id: Some(1),
                tx_id: Some(1),
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            vec[1],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(2),
                tx_id: Some(2),
                amount: Some("2.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_client_missing() {
        let res = parse_transactions_file(&"tests/fixtures/missing_client.csv");

        assert!(res.is_ok());
        let vec: Vec<TransactionLine> = res.unwrap();

        assert_eq!(vec.len(), 2);

        assert_eq!(
            vec[0],
            TransactionLine {
                kind: Some(Deposit),
                client_id: None,
                tx_id: Some(1),
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            vec[1],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(2),
                tx_id: Some(2),
                amount: Some("2.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_tx_missing() {
        let res = parse_transactions_file(&"tests/fixtures/missing_tx.csv");

        assert!(res.is_ok());
        let vec: Vec<TransactionLine> = res.unwrap();

        assert_eq!(vec.len(), 2);

        assert_eq!(
            vec[0],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(1),
                tx_id: None,
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            vec[1],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(2),
                tx_id: Some(2),
                amount: Some("2.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_amount_missing() {
        let res = parse_transactions_file(&"tests/fixtures/missing_amount.csv");

        assert!(res.is_ok());
        let vec: Vec<TransactionLine> = res.unwrap();

        assert_eq!(vec.len(), 2);

        assert_eq!(
            vec[0],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(1),
                tx_id: Some(1),
                amount: None,
            }
        );

        assert_eq!(
            vec[1],
            TransactionLine {
                kind: Some(Deposit),
                client_id: Some(2),
                tx_id: Some(2),
                amount: Some("2.0".to_string()),
            }
        );
    }

    // TODO: try to check error message.
    #[test]
    fn parser_returns_error_if_file_does_not_exist() {
        assert!(parse_transactions_file(&"tests/missing_file.csv").is_err());
    }

    // TODO: try to check error message.
    #[test]
    fn parser_returns_error_if_missing_a_column() {
        assert!(parse_transactions_file(&"tests/missing_column.csv").is_err());
    }
}
