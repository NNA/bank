use crate::models::account_balance::AccountBalance;
use crate::models::AccountId;
use std::collections::HashMap;

pub type BalanceBook = HashMap<AccountId, AccountBalance>;
