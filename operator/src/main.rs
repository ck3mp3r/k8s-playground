mod crd;

mod operator;

use futures::StreamExt;
use k8s_openapi::chrono;
use kube::runtime::controller::Controller;
use kube::{Api, Client};
use log::{error, info};
use operator::shared_state::SharedState;
use operator::subscriber::{error_policy, reconcile};
use std::io::Write;
use std::sync::Arc;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .format(|buf, record| {
            serde_json::to_writer(
                &mut *buf,
                &serde_json::json!({
                    "level": record.level().to_string(),
                    "target": record.target(),
                    "message": record.args(),
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }),
            )
            .unwrap();
            writeln!(buf)
        })
        .init();

    // Initialize Kubernetes client
    let client = Client::try_default().await?;

    // Shared state for the reconciler
    let shared_state = Arc::new(SharedState::new(client.clone()));

    // API for the Subscriber CRD
    let subscribers: Api<crd::subscriber::Subscriber> = Api::all(client);

    // Controller logic
    Controller::new(subscribers, Default::default())
        .run(
            reconcile,            // Reconciliation logic
            error_policy,         // Error handling logic
            shared_state.clone(), // Shared context
        )
        .for_each(|result| async move {
            match result {
                Ok(obj_ref) => info!("Reconciled: {:?}", obj_ref),
                Err(err) => error!("Error during reconciliation: {:?}", err),
            }
        })
        .await;

    Ok(())
}
