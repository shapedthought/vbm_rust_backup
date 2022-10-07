use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Creds {
    pub grant_type: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

impl Creds {
    pub fn new(grant_type: String, username: String, password: String, url: String) -> Creds {
        Creds {
            grant_type,
            username,
            password,
            url,
        }
    }
}

pub struct CredsExtended {
    pub backup_password: String,
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
