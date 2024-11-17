use kube::runtime::controller::Action;
use std::sync::Arc;
use crate::crd::subscriber::Subscriber;
use crate::operator::shared_state::SharedState;

/// Reconciliation function
pub async fn reconcile(
    obj: Arc<Subscriber>,               // The object being reconciled
    _ctx: Arc<SharedState>,             // Shared context
) -> Result<Action, kube::Error> {     // Returns Action or kube::Error
    println!("Reconciling Subscriber: {:?}", obj.metadata.name);

    // Example reconciliation logic
    Ok(Action::requeue(std::time::Duration::from_secs(300))) // Requeue after 5 minutes
}

/// Error policy for handling reconciliation errors
pub fn error_policy(
    obj: Arc<Subscriber>,               // The object associated with the error
    error: &kube::Error,                // The error encountered during reconciliation
    _ctx: Arc<SharedState>,             // Shared context
) -> Action {
    eprintln!(
        "Error reconciling Subscriber {:?}: {:?}",
        obj.metadata.name, error
    );

    // Decide to requeue the reconciliation
    Action::requeue(std::time::Duration::from_secs(60)) // Retry after 1 minute
}

