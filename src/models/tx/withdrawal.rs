use crate::models::raw_tx::RawTransaction;
use crate::models::tx::dispute::DisputeStatus;
use crate::models::tx::TxId;
use crate::models::AccountId;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub struct Withdrawal {
    pub id: TxId,
    pub account_id: AccountId,
    pub amount: Decimal,
    pub dispute_status: DisputeStatus,
}

impl TryFrom<RawTransaction> for Withdrawal {
    type Error = String;

    fn try_from(raw_tx: RawTransaction) -> Result<Self, Self::Error> {
        if let None = raw_tx.client {
            return Err("missing client".to_string());
        }
        if let None = raw_tx.tx {
            return Err("missing tx".to_string());
        }
        if let None = raw_tx.amount {
            return Err("missing amount".to_string());
        }
        let am: Decimal;
        match Decimal::from_str(&raw_tx.amount.unwrap()) {
            Ok(a) => am = a,
            Err(_e) => return Err("invalid amount".to_string()),
        }
        Ok(Withdrawal {
            id: raw_tx.tx.unwrap(),
            account_id: raw_tx.client.unwrap(),
            amount: am,
            dispute_status: DisputeStatus::NotDisputed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::raw_tx::TransactionKind;

    #[test]
    fn withrawal_can_be_created_from_raw_transaction() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Withdrawal),
            client: Some(1),
            tx: Some(42),
            amount: Some("23.2".to_string()),
        };

        let res = Withdrawal::try_from(raw_tx);
        assert!(res.is_ok());
        let wd: Withdrawal = res.unwrap();
        assert_eq!(wd.id, 42);
        assert_eq!(wd.account_id, 1);
        assert_eq!(wd.amount, Decimal::new(232_000, 4));
    }

    #[test]
    fn withdrawal_creation_fails_if_raw_transaction_has_no_tx() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Withdrawal),
            client: Some(1),
            tx: None,
            amount: Some("23.2".to_string()),
        };

        let res = Withdrawal::try_from(raw_tx);
        // assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "missing tx".to_string());
    }

    #[test]
    fn withdrawal_creation_fails_if_raw_transaction_has_no_client() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Withdrawal),
            client: None,
            tx: Some(42),
            amount: Some("23.2".to_string()),
        };

        let res = Withdrawal::try_from(raw_tx);
        // assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "missing client".to_string());
    }

    #[test]
    fn withdrawal_creation_fails_if_raw_transaction_has_no_amount() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Withdrawal),
            client: Some(1),
            tx: Some(42),
            amount: None,
        };

        let res = Withdrawal::try_from(raw_tx);
        // assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "missing amount".to_string());
    }

    #[test]
    fn withdrawal_creation_fails_if_amount_unparsable() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Withdrawal),
            client: Some(1),
            tx: Some(42),
            amount: Some("not_a_number".to_string()),
        };

        let res = Withdrawal::try_from(raw_tx);
        // assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "invalid amount".to_string());
    }
}
