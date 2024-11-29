use std::{fmt::Debug, time::Duration};

use serde::{Deserialize, Serialize};
use temporal_sdk::{ActContext, ActivityError, ActivityOptions, WfContext, WfExitValue};
use temporal_sdk_core::protos::coresdk::{
    activity_result::activity_resolution::Status, AsJsonPayloadExt, FromJsonPayloadExt,
};
use temporal_sdk_core::protos::temporal::api::common::v1::RetryPolicy;

use crate::activities::{deposit, refund, withdraw};
use crate::activities::{DepositArgs, RefundArgs, WithdrawArgs};
use crate::bank::Bank;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyTransferArgs {
    pub bank: Bank,
    pub account_src: u32,
    pub account_tgt: u32,
    pub amount: u32,
}

/// Mostly copied from
/// https://github.com/h7kanna/sdk-core/blob/09b0838c0c19b4ff17a53709ade37f58cb2a0a0f/sdk/src/workflow.rs#L162
async fn execute_activity<In, Out, F>(
    ctx: &WfContext,
    _f: F,
    options: ActivityOptions,
    input: In,
) -> Result<Out, anyhow::Error>
where
    In: AsJsonPayloadExt,
    Out: FromJsonPayloadExt + Debug,
    F: AsyncFn(ActContext, In) -> Result<Out, ActivityError> + Send + Sync + 'static,
{
    let input = In::as_json_payload(&input)?;
    let activity_type = if options.activity_type.is_empty() {
        std::any::type_name::<F>().to_string()
    } else {
        options.activity_type
    };
    let options = ActivityOptions {
        input,
        activity_type,
        ..options
    };
    let activity_resolution = ctx.activity(options).await;

    if activity_resolution.status.is_none() {
        panic!("activity task failed {activity_resolution:?}");
    }

    let status = activity_resolution.status.unwrap();

    match status {
        Status::Completed(success) => Ok(Out::from_json_payload(&success.result.unwrap()).unwrap()),
        Status::Failed(failure) => Err(anyhow::anyhow!("{:?}", failure)),
        Status::Cancelled(reason) => Err(anyhow::anyhow!("{:?}", reason)),
        Status::Backoff(reason) => Err(anyhow::anyhow!("{:?}", reason)),
    }
}

pub async fn money_transfer(ctx: WfContext) -> Result<WfExitValue<Bank>, anyhow::Error> {
    let retry_policy = RetryPolicy {
        maximum_attempts: 5,
        ..Default::default()
    };

    let withdraw_opts = ActivityOptions {
        start_to_close_timeout: Some(Duration::from_secs(30)),
        retry_policy: Some(retry_policy.clone()),
        ..Default::default()
    };

    let deposit_opts = ActivityOptions {
        start_to_close_timeout: Some(Duration::from_secs(30)),
        retry_policy: Some(retry_policy.clone()),
        ..Default::default()
    };

    let refund_opts = ActivityOptions {
        start_to_close_timeout: Some(Duration::from_secs(30)),
        retry_policy: Some(retry_policy),
        ..Default::default()
    };

    let payload = &ctx.get_args()[0];
    let MoneyTransferArgs {
        bank,
        account_src,
        account_tgt,
        amount,
    } = MoneyTransferArgs::from_json_payload(payload)?;

    let withdraw_args = WithdrawArgs {
        bank,
        account: account_src,
        amount,
    };

    let bank: Bank = execute_activity(&ctx, withdraw, withdraw_opts, withdraw_args).await?;

    let deposit_args = DepositArgs {
        bank: bank.clone(),
        account: account_tgt,
        amount,
    };

    let bank = match execute_activity(&ctx, deposit, deposit_opts, deposit_args).await {
        Err(_) => {
            let refund_args = RefundArgs {
                bank,
                account: account_src,
                amount,
            };
            execute_activity(&ctx, refund, refund_opts, refund_args).await?
        }
        Ok(bank) => bank,
    };

    Ok(WfExitValue::Normal(bank))
}
