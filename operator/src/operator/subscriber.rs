use crate::crd::subscriber::Subscriber;
use crate::operator::shared_state::SharedState;
use kube::runtime::controller::Action;
use std::sync::Arc;

pub async fn reconcile(obj: Arc<Subscriber>, ctx: Arc<SharedState>) -> Result<Action, kube::Error> {
    println!("Reconciling Subscriber: {:?}", obj.metadata.name);
    let _ = &ctx.client;

    Ok(Action::requeue(std::time::Duration::from_secs(300)))
}

pub fn error_policy(obj: Arc<Subscriber>, error: &kube::Error, ctx: Arc<SharedState>) -> Action {
    let _ = &ctx.client;
    eprintln!(
        "Error reconciling Subscriber {:?}: {:?}",
        obj.metadata.name, error
    );

    Action::requeue(std::time::Duration::from_secs(60))
}
