use crate::models::account_balance::AccountBalance;
use crate::models::tx::deposit::Deposit;
use crate::models::tx::dispute::Dispute;
use crate::models::tx::withdrawal::Withdrawal;
use crate::models::tx::TxIndex;
use crate::models::AccountId;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ledger {
    balances: AccountIndex,
    transactions: TxIndex,
}

pub type AccountIndex = HashMap<AccountId, AccountBalance>;

impl Ledger {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn find_or_create_account(&mut self, account_id: AccountId) -> &mut AccountBalance {
        if self.balances.contains_key(&account_id) {
            self.balances.get_mut(&account_id).unwrap()
        } else {
            self.balances.insert(account_id, AccountBalance::new());
            self.balances.get_mut(&account_id).unwrap()
        }
    }

    pub fn make_deposit(self: &mut Ledger, deposit: Deposit) -> Result<(), String> {
        let account = self.find_or_create_account(deposit.account_id);
        account.process_deposit(deposit)
    }

    pub fn make_withdrawal(self: &mut Ledger, withdrawal: Withdrawal) -> Result<(), String> {
        let account = self.find_or_create_account(withdrawal.account_id);
        account.process_withdrawal(withdrawal)
    }

    pub fn make_dispute(self: &mut Ledger, dispute: Dispute) -> Result<(), String> {
        let account = self.find_or_create_account(dispute.account_id);
        // let res = account.find_tx(dispute.disputed_tx_id);
        // match res {
        //     Ok(disputed_tx) => dispute.disputed_amount = disputed_tx.amount,
        //     None => return Err("Disputed tx with id not found"),
        // }
        // account.process_dispute(dispute)
        Ok(())
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger {
            balances: AccountIndex::new(),
            transactions: TxIndex::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn find_an_existing_account_given_its_account_id() {
        // Prepare data
        let mut bb = Ledger::new();
        let mut ac = AccountBalance::new();
        ac.available = Decimal::new(10_000, 4);
        bb.balances.insert(1, ac);

        // TODO try impprove this
        // assert_eq!(bb.find_or_create_account(1), &ac); //not allowed
        let found = bb.find_or_create_account(1);
        assert_eq!(found.available, Decimal::new(10_000, 4));
    }

    #[test]
    fn creates_and_returns_default_account_if_missing() {
        // Prepare data
        let mut bb = Ledger::new();

        //Assert
        let created = bb.find_or_create_account(1);
        assert_eq!(created, &mut AccountBalance::new());
    }

    #[test]
    fn find_an_existing_tx_given_its_tx_id() {
        // Prepare data
        let mut bb = Ledger::new();
        let mut ac = AccountBalance::new();
        ac.available = Decimal::new(10_000, 4);
        bb.balances.insert(1, ac);

        // TODO try impprove this
        // assert_eq!(bb.find_or_create_account(1), &ac); //not allowed
        let found = bb.find_or_create_account(1);
        assert_eq!(found.available, Decimal::new(10_000, 4));
    }
}

// pub trait Depositable {
//     fn make_deposit(&mut self, deposit: Deposit);
// }

// impl Depositable for Ledger {
//     fn make_deposit(self: &mut Ledger, deposit: Deposit) {
//         let account = self.find_or_create_account(deposit.account_id);
//         account.process_deposit(deposit);
//     }
// }
