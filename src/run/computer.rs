use crate::models::account_balance::AccountBalance;
use crate::models::balance_book::BalanceBook;
use crate::models::raw_tx::RawTransactionsList;
use crate::models::raw_tx::TransactionKind;
use crate::models::tx::Deposit;
use log::warn;

pub fn compute_balance(txs: RawTransactionsList) -> BalanceBook {
    let mut b_book: BalanceBook = BalanceBook::new();
    for tx in txs {
        // let boxed_tx: Box<_>;
        match tx.kind {
            Some(ref kind) => match kind {
                TransactionKind::Deposit => match Deposit::try_from(tx) {
                    Ok(deposit) => {
                        if let Some(account) = b_book.get_mut(&deposit.account_id) {
                            account.process_deposit(deposit);
                        } else {
                            let id = deposit.account_id;
                            let mut ab = AccountBalance::new();
                            ab.process_deposit(deposit);
                            b_book.insert(id, ab);
                        };
                    }
                    Err(e) => warn!("Invalid deposit : {:?}", e),
                },
                _ => (),
            },
            None => warn!(
                "Unknown transaction {:?}. Skipping this transaction",
                tx.kind
            ),
        }
    }
    b_book
}

// trait Accountable {
//     fn process_transaction(&self);
// }

// impl Accountable for Deposit {
// 	pub fn process_transaction(&mut self, deposit: Deposit) {
//         self.available += deposit.amount as Decimal;
//         //TODO: check overflow
//     }
// }
