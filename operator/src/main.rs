mod crd;
mod operator;

use futures::StreamExt;
use kube::runtime::controller::Controller;
use kube::{Api, Client};
use operator::shared_state::SharedState;
use operator::subscriber::{error_policy, reconcile};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
                Ok(obj_ref) => println!("Reconciled: {:?}", obj_ref),
                Err(err) => eprintln!("Error during reconciliation: {:?}", err),
            }
        })
        .await;

    Ok(())
}
