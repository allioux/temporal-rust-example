use serde::{Deserialize, Serialize};
use temporal_sdk::{ActContext, ActivityError};

use crate::bank::{Bank, BankError};

#[derive(Serialize, Deserialize)]
pub struct WithdrawArgs {
    pub bank: Bank,
    pub account: u32,
    pub amount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct DepositArgs {
    pub bank: Bank,
    pub account: u32,
    pub amount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RefundArgs {
    pub bank: Bank,
    pub account: u32,
    pub amount: u32,
}

fn bank_error_mapping(bank_error: BankError) -> ActivityError {
    match bank_error {
        BankError::InvalidAccount | BankError::NotEnoughMoney => {
            ActivityError::NonRetryable(bank_error.into())
        }
        BankError::TooSoon => ActivityError::from(bank_error),
    }
}

pub async fn withdraw(_ctx: ActContext, args: WithdrawArgs) -> Result<Bank, ActivityError> {
    let WithdrawArgs {
        mut bank,
        account,
        amount,
        ..
    } = args;

    bank.withdraw(account, amount).map_err(bank_error_mapping)?;

    Ok(bank)
}

pub async fn deposit(_ctx: ActContext, args: DepositArgs) -> Result<Bank, ActivityError> {
    let DepositArgs {
        mut bank,
        account,
        amount,
        ..
    } = args;

    bank.deposit(account, amount).map_err(bank_error_mapping)?;

    Ok(bank)
}

pub async fn refund(_ctx: ActContext, args: RefundArgs) -> Result<Bank, ActivityError> {
    let RefundArgs {
        mut bank,
        account,
        amount,
        ..
    } = args;

    bank.deposit(account, amount).map_err(bank_error_mapping)?;

    Ok(bank)
}
