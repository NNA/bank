use std::collections::HashMap;

pub mod deposit;
pub mod dispute;
pub mod withdrawal;

type TxId = u32;

pub type TxIndex = HashMap<TxId, Box<Transaction>>;

#[derive(Debug)]
pub enum Transaction {
    Deposit,
    Withdrawal,
    Dispute,
}
