use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrgData {
    pub id: String,
    pub name: String,
}