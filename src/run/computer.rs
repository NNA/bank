use crate::run::tx::RawTransaction;
use crate::run::tx::RawTransactionsList;
use log::debug;

pub fn compute_balance(txs: RawTransactionsList) {
    for tx in txs {
        // let res = check_transaction(tx);
        // debug!(res);
        // match res {
        //     Some(expr) => expr,
        //     None => expr,
        // }
    }
}

// fn check_transaction(tx: RawTransaction) -> Result<(), std::result::Result> {
//     unimplemented!()
// }

// impl From for Type {
// 	// add code here
// }

// impl TryFrom<RawTransaction> for Transaction {
//     type Error = String;

//     fn try_from(tx: RawTransaction) -> Result<Self, Self::Error> {
//         match s.to_lowercase().as_str() {
//             "local" => Ok(Self::Local),
//             "ci" => Ok(Self::Ci),
//             "production" => Ok(Self::Production),
//             other => Err(format!(
//                 "{} is not a supported environment. Use either `local` or `ci` or `production`.",
//                 other
//             )),
//         }
//     }
// }
