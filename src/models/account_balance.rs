use crate::models::tx::deposit::Deposit;
use crate::models::tx::dispute::Dispute;
use crate::models::tx::withdrawal::Withdrawal;

use rust_decimal::Decimal;

// use crate::models::Decimal;
use log::error;

#[derive(Debug, PartialEq)]
pub struct AccountBalance {
    // TODO change this to no public
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

    /// Total is computed because always equals to available + held
    pub fn total(&self) -> Decimal {
        match self.available.checked_add(self.held) {
            Some(total) => total,
            None => self.available,
        }
    }

    pub fn process_deposit(&mut self, deposit: Deposit) -> Result<Deposit, String> {
        if let Some(total) = self.available.checked_add(deposit.amount) {
            self.available = total;
            Ok(deposit)
        } else {
            let msg = format!(
                "Could not make deposit, maximum available amount of {} reached",
                Decimal::MAX
            );
            error!("{}", msg);
            Err(msg)
        }
    }

    pub fn process_withdrawal(&mut self, withdrawal: Withdrawal) -> Result<Withdrawal, String> {
        match self.available.checked_sub(withdrawal.amount) {
            Some(sub) => match sub.is_sign_positive() {
                true => {
                    self.available = sub;
                    Ok(withdrawal)
                }
                false => {
                    let msg = format!(
                        "Could not make withdrawal, not enough amount available (you asked for {} but only {} are available)",
                        withdrawal.amount,
                        self.available
                    );
                    error!("{}", msg);
                    Err(msg)
                }
            },
            None => {
                let msg = format!(
                    "Could not make withdrawal, minimum available amount of {} reached",
                    Decimal::MIN
                );
                error!("{}", msg);
                Err(msg)
            }
        }
    }

    pub fn process_dispute(&mut self, dispute: Dispute) -> Result<(), String> {
        unimplemented!()
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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    /////////////////////
    // Process Deposit
    /////////////////////

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

    /////////////////////
    // Process Withdrawal
    /////////////////////
    #[test]
    fn process_withdrawal_removes_amount_from_available() {
        // Prepare data
        let mut ab = AccountBalance::new();
        ab.available = Decimal::new(200_000, 4);

        let w = Withdrawal {
            id: 1,
            account_id: 2,
            amount: Decimal::new(150_000, 4),
        };

        // Execute
        let res = ab.process_withdrawal(w);

        // Check
        assert!(res.is_ok());
        assert_eq!(ab.available, Decimal::new(50_000, 4));
    }

    #[test]
    fn process_withdrawal_removes_amount_from_available_even_if_0_available_left_after_withdraw() {
        // Prepare data
        let mut ab = AccountBalance::new();
        ab.available = Decimal::new(200_000, 4);

        let w = Withdrawal {
            id: 1,
            account_id: 2,
            amount: Decimal::new(200_000, 4),
        };

        // Execute
        let res = ab.process_withdrawal(w);

        // Check
        assert!(res.is_ok());
        assert_eq!(ab.available, Decimal::new(0, 4));
    }

    #[test]
    fn process_withdrawal_returns_error_and_leaves_amount_unchanged_if_not_enough_available() {
        // Prepare data
        let mut ab = AccountBalance::new();
        ab.available = Decimal::new(200_000, 4);

        let w = Withdrawal {
            id: 1,
            account_id: 2,
            amount: Decimal::new(400_000, 4),
        };

        // Execute
        let res = ab.process_withdrawal(w);

        // Check
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            format!(
                "Could not make withdrawal, not enough amount available (you asked for {} but only {} are available)",
                Decimal::new(400_000, 4),
                Decimal::new(200_000, 4),
            )
        );
        assert_eq!(ab.available, Decimal::new(200_000, 4));
    }

    #[test]
    fn process_withdrawal_returns_error_if_min_amount_reached() {
        // Prepare data
        let mut ab = AccountBalance::new();
        let just_above_limit = Decimal::MIN + Decimal::new(1_000, 4);
        println!("just_above_limit {:?}", just_above_limit);
        ab.available = just_above_limit;

        let d = Withdrawal {
            id: 1,
            account_id: 2,
            amount: Decimal::new(10_000, 4),
        };

        // Execute
        let res = ab.process_withdrawal(d);

        // Check
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            format!(
                "Could not make withdrawal, minimum available amount of {} reached",
                Decimal::MIN
            )
        );
        assert_eq!(ab.available, just_above_limit);
    }

    /////////////////////
    // Process Dispute
    /////////////////////
    // #[test]
    // fn process_dispute_removes_disputed_amount_from_available_and_increases_held() {
    //     // Prepare data
    //     let mut ab = AccountBalance::new();
    //     ab.available = Decimal::new(200_000, 4);

    //     let w = Withdrawal {
    //         id: 1,
    //         account_id: 2,
    //         amount: Decimal::new(150_000, 4),
    //     };

    //     // Execute
    //     let res = ab.process_withdrawal(w);

    //     // Check
    //     assert!(res.is_ok());
    //     assert_eq!(ab.available, Decimal::new(50_000, 4));
    // }

    /////////////////////
    // total
    /////////////////////
    #[test]
    fn total_makes_sum_available_and_held() {
        // Prepare data
        let ab = AccountBalance {
            available: Decimal::new(1_000, 4),
            held: Decimal::new(1_500, 4),
            locked: false,
        };
        assert_eq!(ab.total(), Decimal::new(2_500, 4));
    }

    #[test]
    fn total_makes_returns_available_if_overflow() {
        // Prepare data
        let ab = AccountBalance {
            available: Decimal::MAX,
            held: Decimal::new(15_500, 4),
            locked: false,
        };

        // Check
        assert_eq!(ab.total(), Decimal::MAX);
    }
}
