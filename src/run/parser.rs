use crate::run::tx::RawTransactionsList;
use csv::Reader;
use log::{debug, trace};
use std::error::Error;
use std::path::Path;

pub fn parse_transactions_file(
    file: &dyn AsRef<Path>,
) -> Result<RawTransactionsList, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file)?;
    let mut txs: RawTransactionsList = RawTransactionsList::new();

    for result in rdr.deserialize() {
        debug!("Parsing line {:?}", result);
        // let record: RawTransaction = result?;
        // txs.push(record);
        match result {
            Ok(record) => {
                trace!("record {:?}", record);
                txs.push(record);
                // let _ = parse_record(record);
            }
            Err(err) => {
                return Err(From::from(err));
            }
        }
    }

    Ok(txs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::tx::RawTransaction;
    use crate::run::tx::TransactionKind::*;

    #[test]
    fn parser_works_if_given_a_correct_file() {
        let res = parse_transactions_file(&"tests/fixtures/regular_tx.csv");

        assert!(res.is_ok());
        let txs: RawTransactionsList = res.unwrap();

        assert_eq!(txs.len(), 5);

        assert_eq!(
            txs[0],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(1),
                tx: Some(1),
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            txs[1],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(2),
                tx: Some(2),
                amount: Some("2.0".to_string()),
            }
        );

        assert_eq!(
            txs[2],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(1),
                tx: Some(3),
                amount: Some("2.0".to_string()),
            }
        );

        assert_eq!(
            txs[3],
            RawTransaction {
                kind: Some(Withdrawal),
                client: Some(1),
                tx: Some(4),
                amount: Some("1.5".to_string()),
            }
        );

        assert_eq!(
            txs[4],
            RawTransaction {
                kind: Some(Withdrawal),
                client: Some(2),
                tx: Some(5),
                amount: Some("3.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_unknown_kind() {
        let res = parse_transactions_file(&"tests/fixtures/unknown_kind.csv");

        assert!(res.is_ok());
        let txs: RawTransactionsList = res.unwrap();

        assert_eq!(txs.len(), 2);

        assert_eq!(
            txs[0],
            RawTransaction {
                kind: None,
                client: Some(1),
                tx: Some(1),
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            txs[1],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(2),
                tx: Some(2),
                amount: Some("2.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_client_missing() {
        let res = parse_transactions_file(&"tests/fixtures/missing_client.csv");

        assert!(res.is_ok());
        let txs: RawTransactionsList = res.unwrap();

        assert_eq!(txs.len(), 2);

        assert_eq!(
            txs[0],
            RawTransaction {
                kind: Some(Deposit),
                client: None,
                tx: Some(1),
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            txs[1],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(2),
                tx: Some(2),
                amount: Some("2.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_tx_missing() {
        let res = parse_transactions_file(&"tests/fixtures/missing_tx.csv");

        assert!(res.is_ok());
        let txs: RawTransactionsList = res.unwrap();

        assert_eq!(txs.len(), 2);

        assert_eq!(
            txs[0],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(1),
                tx: None,
                amount: Some("1.0".to_string()),
            }
        );

        assert_eq!(
            txs[1],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(2),
                tx: Some(2),
                amount: Some("2.0".to_string()),
            }
        );
    }

    #[test]
    fn parser_works_but_says_none_if_amount_missing() {
        let res = parse_transactions_file(&"tests/fixtures/missing_amount.csv");

        assert!(res.is_ok());
        let txs: RawTransactionsList = res.unwrap();

        assert_eq!(txs.len(), 2);

        assert_eq!(
            txs[0],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(1),
                tx: Some(1),
                amount: None,
            }
        );

        assert_eq!(
            txs[1],
            RawTransaction {
                kind: Some(Deposit),
                client: Some(2),
                tx: Some(2),
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
