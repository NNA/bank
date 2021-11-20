use crate::models::raw_tx::RawTransaction;
use crate::models::AccountId;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
// use crate::models::Decimal;

type TxId = u32;

#[derive(Debug)]
pub struct Deposit {
    pub id: TxId,
    pub account_id: AccountId,
    pub amount: Decimal,
}

#[derive(Debug)]
pub struct Withdrawal {
    id: TxId,
    account_id: AccountId,
    amount: Decimal,
}

impl TryFrom<RawTransaction> for Deposit {
    type Error = String;

    fn try_from(raw_tx: RawTransaction) -> Result<Self, Self::Error> {
        if let None = raw_tx.client {
            return Err("A Deposit must have a client".to_string());
        }
        if let None = raw_tx.tx {
            return Err("A Deposit must have a transaction id".to_string());
        }
        if let None = raw_tx.amount {
            return Err("A Deposit must have an amount".to_string());
        }
        let mut am: Decimal;
        match Decimal::from_str(&raw_tx.amount.unwrap()) {
            Ok(a) => am = a,
            Err(e) => return Err("Invalid amount".to_string()),
        }
        Ok(Deposit {
            id: raw_tx.tx.unwrap(),
            account_id: raw_tx.client.unwrap(),
            amount: am,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::raw_tx::TransactionKind;

    #[test]
    fn deposit_can_be_created_from_raw_transaction() {
        // Prepare data
        let raw_tx = RawTransaction {
            kind: Some(TransactionKind::Deposit),
            client: Some(1),
            tx: Some(42),
            amount: Some("23.2".to_string()),
        };

        let res = Deposit::try_from(raw_tx);
        assert!(res.is_ok());
        let deposit: Deposit = res.unwrap();
        assert_eq!(deposit.id, 42);
        assert_eq!(deposit.account_id, 1);
        assert_eq!(deposit.amount, Decimal::new(232_000, 4));
    }
}
