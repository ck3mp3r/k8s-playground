use anyhow::anyhow;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{de::Error, Deserialize, Deserializer, Serialize};

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
    pub id: String,
    #[serde(deserialize_with = "deserialize_identifier")]
    pub identifier: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub helm_chart: HelmChart,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct ProductStatus {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
pub struct HelmChart {
    pub name: String,
    pub registry: String,
    pub version: String,
}

fn deserialize_identifier<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let s = String::deserialize(deserializer)?;
    let re = regex::Regex::new(r"^[a-zA-Z0-9]+:[a-zA-Z0-9]+:v?[0-9]+\.[0-9]+\.[0-9]+$").unwrap();
    if re.is_match(&s) {
        Ok(s)
    } else {
        Err(Error::custom(anyhow!("Invalid identifier format: {}", s)))
    }
}

