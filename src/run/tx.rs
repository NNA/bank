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
    #[serde(rename = "client")]
    pub client_id: Option<u16>,
    #[serde(rename = "tx")]
    pub tx_id: Option<u32>,
    pub amount: Option<String>,
}

pub type RawTransactionsList = Vec<RawTransaction>;
