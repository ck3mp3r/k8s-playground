use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "kemper.buzz",
    version = "v1alpha1",
    kind = "Product",
    plural = "products",
    namespaced,
    status = "ProductStatus"
)]
pub struct ProductSpec {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ProductStatus {
    pub status: String,
}
