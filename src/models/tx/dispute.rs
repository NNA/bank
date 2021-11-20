use crate::models::raw_tx::RawTransaction;
use crate::models::tx::TxId;
use crate::models::AccountId;

#[derive(Debug)]
pub struct Dispute {
    pub account_id: AccountId,
    pub disputed_tx_id: TxId,
}

impl TryFrom<RawTransaction> for Dispute {
    type Error = String;

    fn try_from(raw_tx: RawTransaction) -> Result<Self, Self::Error> {
        if let None = raw_tx.client {
            return Err("missing client".to_string());
        }
        if let None = raw_tx.tx {
            return Err("missing tx".to_string());
        }
        Ok(Dispute {
            account_id: raw_tx.client.unwrap(),
            disputed_tx_id: raw_tx.tx.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::raw_tx::TransactionKind;

    #[test]
    fn dispute_can_be_created_from_raw_transaction() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Dispute),
            client: Some(1),
            tx: Some(42),
            amount: None,
        };

        let res = Dispute::try_from(raw_tx);
        assert!(res.is_ok());
        let dispute: Dispute = res.unwrap();
        assert_eq!(dispute.account_id, 1);
        assert_eq!(dispute.disputed_tx_id, 42);
    }

    #[test]
    fn dispute_creation_fails_if_raw_transaction_has_no_tx() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Dispute),
            client: Some(1),
            tx: None,
            amount: None,
        };

        let res = Dispute::try_from(raw_tx);
        // assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "missing tx".to_string());
    }

    #[test]
    fn dispute_creation_fails_if_raw_transaction_has_no_client() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Dispute),
            client: None,
            tx: Some(42),
            amount: None,
        };

        let res = Dispute::try_from(raw_tx);
        // assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "missing client".to_string());
    }
}
