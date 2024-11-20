use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "kemper.buzz",
    version = "v1alpha1",
    kind = "Subscription",
    namespaced
)]
pub struct SubscriptionSpec {
    pub subscriber: String,
    pub product: String,
}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "kemper.buzz",
    version = "v1alpha1",
    kind = "Product",
    namespaced
)]
pub struct ProductSpec {
    pub name: String,
    pub price: f64,
}
