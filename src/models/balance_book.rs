use crate::models::account_balance::AccountBalance;
use crate::models::tx::Deposit;
use crate::models::AccountId;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BalanceBook {
    content: BalanceBookType,
}

pub type BalanceBookType = HashMap<AccountId, AccountBalance>;

impl BalanceBook {
    pub fn new() -> Self {
        BalanceBook {
            content: BalanceBookType::new(),
        }
    }

    pub fn find_or_create_account(&mut self, account_id: AccountId) -> &mut AccountBalance {
        if self.content.contains_key(&account_id) {
            self.content.get_mut(&account_id).unwrap()
        } else {
            self.content.insert(account_id, AccountBalance::new());
            self.content.get_mut(&account_id).unwrap()
        }
    }

    pub fn make_deposit(self: &mut BalanceBook, deposit: Deposit) {
        let account = self.find_or_create_account(deposit.account_id);
        account.process_deposit(deposit);
    }
}

// pub trait Depositable {
//     fn make_deposit(&mut self, deposit: Deposit);
// }

// impl Depositable for BalanceBook {
//     fn make_deposit(self: &mut BalanceBook, deposit: Deposit) {
//         let account = self.find_or_create_account(deposit.account_id);
//         account.process_deposit(deposit);
//     }
// }
