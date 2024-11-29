use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u32,
    pub balance: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bank {
    accounts: Vec<Account>,
    last_op_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum BankError {
    #[error("Invalid account")]
    InvalidAccount,
    #[error("Action attempted too soon")]
    TooSoon,
    #[error("Not enough money on the bank account")]
    NotEnoughMoney,
}

impl Bank {
    pub fn new(accounts: Vec<Account>) -> Self {
        Bank {
            accounts,
            last_op_at: Utc::now(),
        }
    }

    pub fn deposit(&mut self, account_id: u32, amount: u32) -> Result<(), BankError> {
        if (Utc::now() - self.last_op_at).num_seconds() < 5 {
            Err(BankError::TooSoon)
        } else {
            let account = self
                .accounts
                .iter_mut()
                .find(|account| account.id == account_id)
                .ok_or(BankError::InvalidAccount)?;

            account.balance += amount;
            self.last_op_at = Utc::now();
            Ok(())
        }
    }

    pub fn withdraw(&mut self, account_id: u32, amount: u32) -> Result<(), BankError> {
        if (Utc::now() - self.last_op_at).num_seconds() < 5 {
            Err(BankError::TooSoon)
        } else {
            let account = self
                .accounts
                .iter_mut()
                .find(|account| account.id == account_id)
                .ok_or(BankError::InvalidAccount)?;

            if amount <= account.balance {
                account.balance -= amount;
                self.last_op_at = Utc::now();
                Ok(())
            } else {
                Err(BankError::NotEnoughMoney)
            }
        }
    }
}
