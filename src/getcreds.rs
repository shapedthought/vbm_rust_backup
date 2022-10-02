use std::fs;

use anyhow::Result;

use crate::models::credsmodel::Creds;

pub fn get_creds() -> Result<Creds> {
    let file = fs::read_to_string("creds.json")?;
    let mut creds: Creds = serde_json::from_str(&file)?;
    let pass_bytes = base64::decode(creds.password.as_bytes())?;
    creds.password = String::from_utf8_lossy(&pass_bytes).to_string();
    Ok(creds)
}