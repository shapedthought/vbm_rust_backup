use std::{fs::{self, File}, io::Write};

use anyhow::Result;
use dialoguer::{Input, Password};

use crate::models::credsmodel::Creds;

pub fn get_creds() -> Result<Creds> {
    let file = fs::read_to_string("creds.json")?;
    let mut creds: Creds = serde_json::from_str(&file)?;
    let pass_bytes = base64::decode(creds.password.as_bytes())?;
    creds.password = String::from_utf8_lossy(&pass_bytes).to_string();
    Ok(creds)
}

pub fn create_creds() {

    let username : String = Input::new()
        .with_prompt("Username")
        .interact_text()
        .unwrap();
    
    let url : String = Input::new()
        .with_prompt("Address")
        .interact_text()
        .unwrap();

    let password = Password::new()
        .with_prompt("Enter password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()
        .unwrap();
    
    let b64 = base64::encode(password.as_bytes());

    let creds = Creds {
        grant_type: "password".to_string(),
        username,
        password: b64,
        url
    };

    let mut file1 = File::create("creds.json").unwrap();
    let string1 = serde_json::to_string(&creds).unwrap();
    file1.write_all(string1.as_bytes()).unwrap();
    println!("Done")
}