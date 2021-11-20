use crate::models::tx::TxId;
use crate::models::AccountId;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Withdrawal {
    id: TxId,
    account_id: AccountId,
    amount: Decimal,
}
