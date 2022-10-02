use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyModel {
    #[serde(rename = "type")]
    pub type_field: String,
    pub use_internet_proxy: bool,
    pub internet_proxy_type: String,
    pub id: String,
    pub host_name: String,
    pub description: String,
    pub port: i64,
    pub threads_number: i64,
    pub enable_network_throttling: bool,
    pub status: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: SelfField,
    pub repositories: Repositories,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repositories {
    pub href: String,
}
