use crate::crd::subscriber::{Subscriber, SubscriberStatus};
use crate::operator::constants::field_manager;
use crate::operator::shared_state::SharedState;
use kube::api::{Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Api, ResourceExt};
use serde_json::json;
use std::sync::Arc;

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
            Ok(Action::requeue(std::time::Duration::from_secs(300)))
        }
        Err(err) => {
            log::error!(
                "Error reconciling Subscriber {:?}: {:?}",
                obj.metadata.name,
                err
            );
            Err(err) // Propagate the error to retry
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
