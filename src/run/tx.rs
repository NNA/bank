use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RawTransaction {
    #[serde(rename = "type")]
    #[serde(deserialize_with = "csv::invalid_option")]
    pub kind: Option<TransactionKind>,
    pub client: Option<u16>,
    pub tx: Option<u32>,
    pub amount: Option<String>,
}

pub type RawTransactionsList = Vec<RawTransaction>;

#[derive(Debug)]
pub struct Deposit {
    pub id: u32,
    pub account_id: u16,
    pub amount: u32,
}

#[derive(Debug)]
pub struct Withdrawal {
    id: u32,
    account_id: u16,
    amount: u32,
}

pub type Decimal = u64;
pub type AccountId = u16;

#[derive(Debug)]
pub struct AccountBalance {
    // total will be computed
    pub available: Decimal,
    pub held: Decimal,
    pub locked: bool,
}

pub type BalanceBook = HashMap<AccountId, AccountBalance>;
