use crate::models::tx::deposit::Deposit;
use crate::models::tx::withdrawal::Withdrawal;
use std::collections::HashMap;

pub mod deposit;
pub mod dispute;
pub mod withdrawal;

pub type TxId = u32;

// #[derive(Debug, PartialEq)]
// pub enum RememberableTransaction {
//     Deposit,
//     Withdrawal,
// }
