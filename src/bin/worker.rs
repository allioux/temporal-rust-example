use std::{any::type_name_of_val, error::Error, str::FromStr, sync::Arc};

use temporal_sdk::{self, sdk_client_options, Worker};
use temporal_sdk_core::{init_worker, CoreRuntime, Url};
use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};

use lib::{
    activities::{deposit, refund, withdraw},
    workflow::money_transfer,
};

pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;
    let client = server_options.connect("default", None).await?;
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("task_queue")
        .worker_build_id("rust-sdk")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), "task_queue");

    worker.register_activity(type_name_of_val(&withdraw), withdraw);
    worker.register_activity(type_name_of_val(&deposit), deposit);
    worker.register_activity(type_name_of_val(&refund), refund);

    worker.register_wf(type_name_of_val(&money_transfer), money_transfer);

    worker.run().await?;

    Ok(())
}
