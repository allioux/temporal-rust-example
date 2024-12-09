use std::{any::type_name_of_val, error::Error, str::FromStr, sync::Arc};

use temporal_client::TlsConfig;
use temporal_sdk::{self, sdk_client_options, Worker};
use temporal_sdk_core::{init_worker, CoreRuntime, Url};
use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};

use lib::{
    activities::{deposit, refund, withdraw},
    workflow::money_transfer,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "";

    let server_options =
        sdk_client_options(Url::from_str("https://eu-west-2.aws.api.temporal.io:7233")?)
            .api_key(Some(api_key.to_string()))
            //.client_name("sugar-temporal-client")
            //.client_name("transcoder-test")
            //.identity("sugar temporal client".to_owned())
            //.identity("transcoder-test".to_owned())
            .tls_cfg(TlsConfig::default())
            .build()?;

    let client = server_options.connect("sugar-test.yyja2", None).await?;
    /*

    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        //.namespace("sugar-test.yyja2")
        .task_queue("task_queue")
        .worker_build_id("rust-sdk")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), "task_queue");

    worker.register_activity(type_name_of_val(&withdraw), withdraw);
    worker.register_activity(type_name_of_val(&deposit), deposit);
    worker.register_activity(type_name_of_val(&refund), refund);

    worker.register_wf(type_name_of_val(&money_transfer), money_transfer);

    worker.run().await?;*/

    Ok(())
}
