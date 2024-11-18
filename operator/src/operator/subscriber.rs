use crate::crd::subscriber::{Subscriber, SubscriberStatus};
use crate::operator::constants::field_manager;
use crate::operator::shared_state::SharedState;
use kube::runtime::controller::Action;
use kube::{Api, ResourceExt};
use serde_json::json;
use std::sync::Arc;

pub async fn reconcile(obj: Arc<Subscriber>, ctx: Arc<SharedState>) -> Result<Action, kube::Error> {
    println!("Reconciling Subscriber: {:?}", obj.metadata.name);

    let namespace = obj.namespace().unwrap_or_else(|| "default".to_string());
    let api: Api<Subscriber> = Api::namespaced(ctx.client.clone(), &namespace);

    let status = SubscriberStatus {
        status: "Reconciled".to_string(),
    };

    let payload = json!({
        "metadata": {
            "name": obj.name_any(),
            "namespace": namespace,
        },
        "status": status
    });

    let patch_params = kube::api::PatchParams::apply(field_manager::SUBSCRIBER);

    let res = api
        .patch_status(
            &obj.name_any(),
            &patch_params,
            &kube::api::Patch::Merge(payload),
        )
        .await;

    match res {
        Ok(_) => {
            println!(
                "Successfully updated status for Subscriber {:?}",
                obj.metadata.name
            );
            Ok(Action::requeue(std::time::Duration::from_secs(300)))
        }
        Err(err) => {
            eprintln!(
                "Error reconciling Subscriber {:?}: {:?}",
                obj.metadata.name, err
            );
            Err(err) // Propagate the error to retry
        }
    }
}

pub fn error_policy(obj: Arc<Subscriber>, error: &kube::Error, _ctx: Arc<SharedState>) -> Action {
    eprintln!(
        "Error reconciling Subscriber {:?}: {:?}",
        obj.metadata.name, error
    );

    Action::requeue(std::time::Duration::from_secs(60))
}
