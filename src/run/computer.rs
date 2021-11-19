use crate::run::tx::AccountBalance;
use crate::run::tx::BalanceBook;
use crate::run::tx::Deposit;
use crate::run::tx::RawTransaction;
use crate::run::tx::RawTransactionsList;
use crate::run::tx::TransactionKind;
use log::{debug, warn};

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
                            let ab = AccountBalance::new();
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

impl AccountBalance {
    pub fn new() -> Self {
        AccountBalance {
            ..Default::default()
        }
    }
    pub fn process_deposit(&self, deposit: Deposit) {
        debug!("processing deposit {:?}", deposit);
    }
}

impl Default for AccountBalance {
    fn default() -> Self {
        AccountBalance {
            locked: false,
            available: 0,
            held: 0,
        }
    }
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
