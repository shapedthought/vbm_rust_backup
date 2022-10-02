use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoModel {
    pub object_storage_cache_path: Option<String>,
    pub object_storage_encryption_enabled: Option<bool>,
    pub is_out_of_sync: bool,
    pub capacity_bytes: i64,
    pub free_space_bytes: i64,
    pub id: String,
    pub name: String,
    pub description: String,
    pub retention_type: String,
    pub retention_period_type: String,
    pub yearly_retention_period: Option<String>,
    pub retention_frequency_type: String,
    pub monthly_time: Option<String>,
    pub monthly_daynumber: Option<String>,
    pub monthly_dayofweek: Option<String>,
    pub proxy_id: String,
    pub is_long_term: Option<bool>,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Selffield,
    pub proxy: Proxy,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selffield {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proxy {
    pub href: String,
}
