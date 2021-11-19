use serde::Deserialize;

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
