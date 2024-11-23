use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "kemper.buzz",
    version = "v1alpha1",
    kind = "Subscriber",
    plural = "subscribers",
    namespaced,
    status = "SubscriberStatus"
)]
pub struct SubscriberSpec {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct SubscriberStatus {
    pub status: String,
}
