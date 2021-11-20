use crate::models::tx::Deposit;
use rust_decimal::Decimal;
// use crate::models::Decimal;
use log::error;

#[derive(Debug, PartialEq)]
pub struct AccountBalance {
    // total will be computed
    pub available: Decimal,
    pub held: Decimal,
    pub locked: bool,
}

impl AccountBalance {
    pub fn new() -> Self {
        AccountBalance {
            ..Default::default()
        }
    }
    pub fn process_deposit(&mut self, deposit: Deposit) {
        if let Some(total) = self.available.checked_add(deposit.amount as Decimal) {
            self.available = total;
        } else {
            error!(
                "Could not make deposit, maximum amount of {} reached",
                Decimal::MAX
            );
        }
    }
}

impl Default for AccountBalance {
    fn default() -> Self {
        AccountBalance {
            locked: false,
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
        }
    }
}
