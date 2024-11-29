use std::{any::type_name_of_val, error::Error, str::FromStr};

use temporal_client::{self, WorkflowOptions};
use temporal_sdk::{self, sdk_client_options};
use temporal_sdk_core::{protos::coresdk::AsJsonPayloadExt, Url, WorkflowClientTrait};

use lib::{
    bank::{Account, Bank},
    workflow::{money_transfer, MoneyTransferArgs},
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;
    let client = server_options.connect("default", None).await?;

    let wf_type = type_name_of_val(&money_transfer).to_owned();
    let wf_id = "bank_wf".to_string();
    let wf_options = WorkflowOptions::default();

    let bank = Bank::new(vec![
        Account {
            id: 0,
            balance: 320,
        },
        Account {
            id: 1,
            balance: 380,
        },
    ]);

    let workflow_args = MoneyTransferArgs {
        bank,
        account_src: 0,
        account_tgt: 1,
        amount: 200,
    };

    let payload = workflow_args.as_json_payload()?;

    client
        .start_workflow(
            vec![payload],
            "task_queue".to_string(),
            wf_id,
            wf_type,
            None,
            wf_options,
        )
        .await?;

    Ok(())
}
