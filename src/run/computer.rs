use crate::models::ledger::Ledger;
use crate::models::raw_tx::RawTransaction;
use crate::models::raw_tx::RawTransactionsList;
use crate::models::raw_tx::TransactionKind;
use crate::models::tx::deposit::Deposit;
use crate::models::tx::withdrawal::Withdrawal;
use log::warn;

// use crate::models::ledger::Depositable;

// use crate::models::Decimal;

pub fn compute_ledger(txs: RawTransactionsList) -> Ledger {
    let mut ledger: Ledger = Ledger::new();
    for tx in txs {
        // let boxed_tx: Box<_>;
        match tx.kind {
            Some(ref kind) => match kind {
                TransactionKind::Deposit => match Deposit::try_from(tx) {
                    Ok(deposit) => match ledger.make_deposit(deposit) {
                        Ok(_) => (),
                        Err(s) => warn!("{}", s),
                    },
                    Err(e) => warn!("Invalid deposit : {:?}", e),
                },
                TransactionKind::Withdrawal => match Withdrawal::try_from(tx) {
                    Ok(withdrawal) => match ledger.make_withdrawal(withdrawal) {
                        Ok(_) => (),
                        Err(s) => warn!("{}", s),
                    },
                    Err(e) => warn!("Invalid withdrawal : {:?}", e),
                },

                // NEW - not working
                // TransactionKind::Deposit => match Transaction::<Deposit>::new(tx) {
                //     Some(_generic_tx) => {
                //     }
                //     None => warn!("Invalid transaction : "),
                // },

                // OLD - working but verbose
                // TransactionKind::Deposit => match Deposit::try_from(tx) {
                //     Ok(deposit) => {
                //         if let Some(account) = b_book.get_mut(&deposit.account_id) {
                //             account.process_deposit(deposit);
                //         } else {
                //             let id = deposit.account_id;
                //             let mut ab = AccountBalance::new();
                //             ab.process_deposit(deposit);
                //             b_book.insert(id, ab);
                //         };
                //     }
                //     Err(e) => warn!("Invalid deposit : {:?}", e),
                // },
                _ => (),
            },
            None => warn!(
                "Unknown transaction {:?}. Skipping this transaction",
                tx.kind
            ),
        }
    }

    ledger
}

pub trait Accountable {
    fn process_transaction(&self, ledger: &mut Ledger);
}

// impl Accountable for Deposit {
//     type Error = String;

//     fn process_transaction(&self, book: &mut BalanceBook) -> Result<Self, Self::Error> {
//         if let Some(new_total) = book.available.checked_add(self.amount as Decimal) {
//             self.available = new_total;
//         } else {
//             error!(
//                 "Could not make deposit, maximum amount of {} reached",
//                 Decimal::MAX
//             );
//         }
//     }
// }

pub struct Transaction<T: Accountable> {
    // TODO Add topic to the Inbound
    pub subject: T,
}

impl<T> Transaction<T>
where
    T: Accountable + std::convert::TryFrom<RawTransaction>,
{
    pub fn new(raw_tx: RawTransaction) -> Option<Self> {
        // match <T>::try_from(raw_tx) {
        //     Ok(tx) => Ok(Transaction { subject: tx }),
        //     Err(e) => Err(From::from(e)),
        // }
        <T>::try_from(raw_tx)
            .ok()
            .and_then(|t| Some(Transaction { subject: t }))
    }

    pub fn update_ledger(&self, _ledger: &mut Ledger) {
        // if let Some(account) = b_book.get_mut(self.subject.account_id) {
        //     self.subject.process_transaction(deposit);
        // } else {
        //     let id = deposit.account_id;
        //     let mut ab = AccountBalance::new();
        //     ab.process_deposit(deposit);
        //     b_book.insert(id, ab);
        // };
    }
}

// impl Accountable for Deposit {
// 	pub fn process_transaction(&mut self, deposit: Deposit) {
//         self.available += deposit.amount as Decimal;
//         //TODO: check overflow
//     }
// }
