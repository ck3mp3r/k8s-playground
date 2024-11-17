mod crd;
mod operator;

use kube::{Api, Client};
use kube::runtime::controller::Controller;
use futures::StreamExt;
use std::sync::Arc;
use operator::shared_state::SharedState;
use operator::subscriber::{reconcile, error_policy};

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
            reconcile,                         // Reconciliation logic
            error_policy,                      // Error handling logic
            shared_state.clone(),              // Shared context
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

