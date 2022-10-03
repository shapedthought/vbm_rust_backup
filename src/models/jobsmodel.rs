use serde::{self, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupJobs {
    #[serde(rename(serialize = "_links", deserialize = "_links"))]
    pub links: Links,
    pub backup_type: String,
    pub description: String,
    pub id: String,
    pub is_enabled: bool,
    pub name: String,
    pub repository_id: String,
    pub schedule_policy: SchedulePolicy,
    pub selected_items: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupJobs2<T, U, V> {
    #[serde(rename(serialize = "_links", deserialize = "_links"))]
    pub backup_type: T,
    pub description: U,
    pub id: U,
    pub is_enabled: V,
    pub name: U,
    pub repository_id: U,
    pub schedule_policy: SchedulePolicy,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupJobSave {
    pub backup_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_items: Option<Vec<Value>>,
    pub description: String,
    pub id: String,
    pub is_enabled: bool,
    pub name: String,
    pub repository_id: String,
    pub schedule_policy: SchedulePolicy,
    pub run_now: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchedulePolicy {
    pub backup_window_enabled: bool,
    pub daily_time: String,
    pub daily_type: String,
    pub retry_enabled: bool,
    pub retry_number: u8,
    pub retry_wait_interval: u8,
    pub schedule_enabled: bool,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub backup_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub selected_items: Href,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrgBackups {
    pub backup_org: String,
    pub backup_org_id: String,
    pub backup_jobs: BackupJobSave,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Href {
    pub href: String,
}
