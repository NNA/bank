use crate::models::tx::deposit::Deposit;
use rust_decimal::Decimal;
// use crate::models::Decimal;
use log::error;

#[derive(Debug, PartialEq)]
pub struct AccountBalance {
    // TODO change this to no public
    pub available: Decimal,
    pub held: Decimal,
    pub locked: bool,
    // total will be computed
}

impl AccountBalance {
    pub fn new() -> Self {
        AccountBalance {
            ..Default::default()
        }
    }

    pub fn process_deposit(&mut self, deposit: Deposit) -> Result<(), String> {
        if let Some(total) = self.available.checked_add(deposit.amount as Decimal) {
            self.available = total;
            Ok(())
        } else {
            let msg = format!(
                "Could not make deposit, maximum available amount of {} reached",
                Decimal::MAX
            );
            error!("{}", msg);
            Err(msg)
        }
    }

    // pub fn process_withrawal(&mut self, withdrawal: Withdrawal) {
    //     if let Some(total) = self.available {
    //         self.available = total;
    //     } else {
    //         error!(
    //             "Could not make withdrawal, maximum amount of {} reached",
    //             Decimal::MAX
    //         );
    //     }
    // }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn process_deposit_adds_amount_to_available() {
        // Prepare data
        let mut ab = AccountBalance::new();
        ab.available = Decimal::new(10_000, 4);

        let d = Deposit {
            id: 1,
            account_id: 2,
            amount: Decimal::new(5_000, 4),
        };

        // Execute
        let res = ab.process_deposit(d);

        // Check
        assert!(res.is_ok());
        assert_eq!(ab.available, Decimal::new(15_000, 4));
    }

    #[test]
    fn process_deposit_returns_error_if_max_amount_reached() {
        // Prepare data
        let mut ab = AccountBalance::new();
        let just_under_limit = Decimal::MAX - Decimal::new(1_000, 4);
        println!("just_under_limit {:?}", just_under_limit);
        ab.available = just_under_limit;

        let d = Deposit {
            id: 1,
            account_id: 2,
            amount: Decimal::new(10_000, 4),
        };

        // Execute
        let res = ab.process_deposit(d);

        // Check
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            format!(
                "Could not make deposit, maximum available amount of {} reached",
                Decimal::MAX
            )
        );
        assert_eq!(ab.available, just_under_limit);
    }
}
