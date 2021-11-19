use crate::models::raw_tx::RawTransaction;
use crate::models::AccountId;
use crate::models::Decimal;

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
            return Err("A Deposit must have a valid transaction id".to_string());
        }
        Ok(Deposit {
            id: raw_tx.tx.unwrap(),
            account_id: raw_tx.client.unwrap(),
            amount: 10,
        })
    }
}
