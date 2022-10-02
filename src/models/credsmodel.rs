use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Creds {
    pub grant_type: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredsResponse {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub expires_in: i32,
}