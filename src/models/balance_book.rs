use crate::models::account_balance::AccountBalance;
use crate::models::tx::deposit::Deposit;
use crate::models::AccountId;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BalanceBook {
    content: BalanceBookType,
}

pub type BalanceBookType = HashMap<AccountId, AccountBalance>;

impl BalanceBook {
    pub fn new() -> Self {
        Default::default()
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

impl Default for BalanceBook {
    fn default() -> Self {
        BalanceBook {
            content: BalanceBookType::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn balance_book_can_find_an_existing_account_given_its_account_id() {
        // Prepare data
        let mut bb = BalanceBook::new();
        let mut ac = AccountBalance::new();
        ac.available = Decimal::new(10_000, 4);
        bb.content.insert(1, ac);

        // TODO try impprove this
        // assert_eq!(bb.find_or_create_account(1), &ac); //not allowed
        let found = bb.find_or_create_account(1);
        assert_eq!(found.available, Decimal::new(10_000, 4));
    }

    #[test]
    fn balance_book_creates_and_returns_default_account_if_missing() {
        // Prepare data
        let mut bb = BalanceBook::new();

        //Assert
        let created = bb.find_or_create_account(1);
        assert_eq!(created, &mut AccountBalance::new());
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
