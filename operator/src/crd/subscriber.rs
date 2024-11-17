use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "example.com",
    version = "v1",
    kind = "Subscriber",
    plural = "subscribers",
    namespaced
)]
pub struct SubscriberSpec {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubscriberStatus {
    pub status: String,
}
