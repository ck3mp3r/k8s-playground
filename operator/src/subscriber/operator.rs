use crate::subscriber::constants::field_manager;
use crate::subscriber::crd::{Subscriber, SubscriberStatus};
use crate::subscriber::shared_state::SharedState;
use futures::StreamExt;
use kube::api::{Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, ResourceExt};
use log::{error, info};
use serde_json::json;
use std::sync::Arc;

pub async fn controller(client: kube::Client) {
    let subscribers: Api<Subscriber> = Api::all(client.clone());
    let shared_state = Arc::new(SharedState {
        client: client.clone(),
    });

    Controller::new(subscribers, Default::default())
        .run(reconcile, error_policy, shared_state.clone())
        .for_each(|result| async move {
            match result {
                Ok(obj_ref) => info!("Reconciled: {:?}", obj_ref),
                Err(err) => error!("Error during reconciliation: {:?}", err),
            }
        })
        .await;
}

pub async fn reconcile(obj: Arc<Subscriber>, ctx: Arc<SharedState>) -> Result<Action, kube::Error> {
    log::info!("Reconciling Subscriber: {:?}", obj.metadata.name);

    let namespace = obj.namespace().unwrap_or_else(|| "default".to_string());
    let api: Api<Subscriber> = Api::namespaced(ctx.client.clone(), &namespace);

    let status = SubscriberStatus {
        status: "Reconciled".to_string(),
    };

    let res = api
        .patch_status(
            &obj.name_any(),
            &PatchParams::apply(field_manager::SUBSCRIBER),
            &Patch::Merge(json!({
                "metadata": {
                    "name": obj.name_any(),
                    "namespace": namespace,
                },
                "status": status
            })),
        )
        .await;

    match res {
        Ok(_) => {
            log::info!(
                "Successfully updated status for Subscriber {:?}",
                obj.metadata.name
            );
            Ok(Action::await_change())
        }
        Err(err) => {
            log::error!(
                "Error reconciling Subscriber {:?}: {:?}",
                obj.metadata.name,
                err
            );
            Err(err)
        }
    }
}

pub fn error_policy(obj: Arc<Subscriber>, error: &kube::Error, _ctx: Arc<SharedState>) -> Action {
    log::error!(
        "Error reconciling Subscriber {:?}: {:?}",
        obj.metadata.name,
        error
    );

    Action::requeue(std::time::Duration::from_secs(60))
}
