use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::{Context, Result};
use dialoguer::{Input, Password};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use base64;

use crate::models::credsmodel::{CredsExtended, ReadCreds};

pub fn get_creds() -> Result<CredsExtended> {
    let file = fs::read_to_string("creds.json")?;
    let creds: ReadCreds = serde_json::from_str(&file)?;
    // let pass_bytes = base64::decode(creds.password.as_bytes())?;

    let bu_password = Password::new()
        .with_prompt("Enter Backup password")
        .interact()?;

    let user_name_base64 = base64::encode(&creds.username);

    let password_extended = format!("{}:{}", user_name_base64, bu_password);

    let mc = new_magic_crypt!(&password_extended, 256);

    let decrypt_string = mc
        .decrypt_base64_to_string(&creds.password)
        .with_context(|| "Wrong password!".to_string())?;

    let creds_extended = CredsExtended {
        backup_password: bu_password,
        grant_type: creds.grant_type,
        username: creds.username,
        url: creds.url,
        password: decrypt_string,
        port: creds.port,
        api_version: creds.api_version,
    };

    Ok(creds_extended)
}

pub fn create_creds() -> Result<()> {
    let username: String = Input::new().with_prompt("Username").interact_text()?;

    let url: String = Input::new().with_prompt("Address").interact_text()?;

    let port: u16 = Input::new()
        .with_prompt("Select Port")
        .default(4443)
        .interact_text()?;

    let api_version: String = Input::new()
        .with_prompt("Select API version")
        .default("v6".into())
        .interact_text()?;

    let password = Password::new()
        .with_prompt("Enter VB365 password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?;

    let bu_password = Password::new()
        .with_prompt("Enter Backup password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?;

    let user_name_base64 = base64::encode(username.as_bytes());

    let password_extended = format!("{}:{}", user_name_base64, bu_password);

    let mc = new_magic_crypt!(password_extended, 256);
    let base64 = mc.encrypt_str_to_base64(password);

    let creds = ReadCreds {
        grant_type: "password".to_string(),
        username,
        password: base64,
        url,
        port,
        api_version,
    };

    let mut file1 = File::create("creds.json")?;
    let string1 = serde_json::to_string(&creds)?;
    file1.write_all(string1.as_bytes())?;
    println!("Done");

    Ok(())
}
