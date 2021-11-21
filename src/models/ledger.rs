use crate::models::account_balance::AccountBalance;
use crate::models::tx::deposit::Deposit;
use crate::models::tx::dispute::Dispute;
use crate::models::tx::withdrawal::Withdrawal;

// use crate::models::tx::RememberableTransaction;

use crate::models::tx::TxId;
use crate::models::AccountId;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ledger {
    pub balances: AccountIndex,
    // transactions: TxIndex,
    deposits: HashMap<TxId, Deposit>,
    withdrawals: HashMap<TxId, Withdrawal>,
}

// pub trait Disputable {
//     fn disputable_ref(&self) -> TxId;
// }

// impl Disputable for Deposit {
//     fn disputable_ref(&self) -> TxId {
//         self.id
//     }
// }

// impl Disputable for Withdrawal {
//     fn disputable_ref(&self) -> TxId {
//         self.id
//     }
// }

// pub type TxIndex = HashMap<TxId, Box<dyn Disputable>>;
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

    // pub fn find_tx(&self, tx_id: TxId) -> Result<&Box<dyn Disputable>, String> {
    //     if self.transactions.contains_key(&tx_id) {
    //         Ok(self.transactions.get(&tx_id).unwrap())
    //     } else {
    //         Err("Not found".to_string())
    //     }
    // }

    pub fn find_tx_in_deposits(&mut self, deposit_id: TxId) -> Option<&mut Deposit> {
        self.deposits.get_mut(&deposit_id)
    }

    // TODO: test
    pub fn find_tx_in_withdrawals(&mut self, withdrawal_id: TxId) -> Option<&mut Withdrawal> {
        self.withdrawals.get_mut(&withdrawal_id)
    }

    // TODO: test
    pub fn make_deposit(self: &mut Ledger, deposit: Deposit) -> Result<(), String> {
        let account = self.find_or_create_account(deposit.account_id);
        let processed_deposit = account.process_deposit(deposit)?;
        self.store_deposit(processed_deposit)
    }

    // TODO: test
    pub fn store_deposit(self: &mut Ledger, deposit: Deposit) -> Result<(), String> {
        if self.deposits.contains_key(&deposit.id) {
            return Err("The deposit already exists...".to_string());
        } else {
            self.deposits.insert(deposit.id, deposit);
            Ok(())
        }
    }

    // TODO: test
    pub fn make_withdrawal(self: &mut Ledger, withdrawal: Withdrawal) -> Result<(), String> {
        let account = self.find_or_create_account(withdrawal.account_id);
        let processed_withdrawal = account.process_withdrawal(withdrawal)?;
        self.store_withdrawal(processed_withdrawal)
    }

    // TODO: test
    pub fn store_withdrawal(self: &mut Ledger, withdrawal: Withdrawal) -> Result<(), String> {
        if self.withdrawals.contains_key(&withdrawal.id) {
            return Err("The withdrawal already exists...".to_string());
        } else {
            self.withdrawals.insert(withdrawal.id, withdrawal);
            Ok(())
        }
    }

    // TODO: test
    pub fn complete_dispute(self: &mut Ledger, mut dispute: Dispute) -> Result<(), String> {
        // let account = self.find_or_create_account(dispute.account_id);
        if let Some(deposit) = self.find_tx_in_deposits(dispute.disputed_tx_id) {
            if deposit.account_id != dispute.account_id {
                return Err("The disputed tx client is not the dispute client".to_string());
            } else {
                dispute.disputed_amount = Some(deposit.amount);
            }
        } else {
            if let Some(withdrawal) = self.find_tx_in_withdrawals(dispute.disputed_tx_id) {
                if withdrawal.account_id != dispute.account_id {
                    return Err("The disputed tx client is not the dispute client".to_string());
                } else {
                    dispute.disputed_amount = Some(withdrawal.amount);
                }
            }
        }
        Ok(())
    }

    // TODO: test
    pub fn make_dispute(self: &mut Ledger, complete_dispute: Dispute) -> Result<(), String> {
        let account = self.find_or_create_account(complete_dispute.account_id);
        account.process_dispute(complete_dispute)
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger {
            balances: AccountIndex::new(),
            // transactions: TxIndex::new(),
            deposits: HashMap::new(),
            withdrawals: HashMap::new(),
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
        let mut ledger = Ledger::new();
        let mut ac = AccountBalance::new();
        ac.available = Decimal::new(10_000, 4);
        ledger.balances.insert(1, ac);

        // TODO try impprove this
        // assert_eq!(ledger.find_or_create_account(1), &ac); //not allowed
        let found = ledger.find_or_create_account(1);
        assert_eq!(found.available, Decimal::new(10_000, 4));
    }

    #[test]
    fn creates_and_returns_default_account_if_missing() {
        // Prepare data
        let mut ledger = Ledger::new();

        //Assert
        let created = ledger.find_or_create_account(1);
        assert_eq!(created, &mut AccountBalance::new());
    }

    #[test]
    fn find_an_existing_deposit_given_its_tx_id() {
        // Prepare data
        let mut ledger = Ledger::new();
        let deposit = Deposit {
            id: 1,
            account_id: 3324,
            amount: Decimal::new(10_000, 4),
        };
        ledger.deposits.insert(1, deposit);

        // TODO try impprove this
        let found = ledger.find_tx_in_deposits(1).unwrap();
        assert_eq!(found.account_id, 3324);
        assert!(ledger.find_tx_in_deposits(2).is_none());
    }

    #[test]
    fn find_an_existing_withdrawal_given_its_tx_id() {
        // Prepare data
        let mut ledger = Ledger::new();
        let withdrawal = Withdrawal {
            id: 1,
            account_id: 3321,
            amount: Decimal::new(10_000, 4),
        };
        ledger.withdrawals.insert(1, withdrawal);

        let found = ledger.find_tx_in_withdrawals(1).unwrap();
        assert_eq!(found.account_id, 3321);
        assert!(ledger.find_tx_in_withdrawals(2).is_none());
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
