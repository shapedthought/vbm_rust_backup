use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::{Context, Result};
use dialoguer::{Input, Password};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::models::credsmodel::{Creds, CredsExtended};

pub fn get_creds() -> Result<CredsExtended> {
    let file = fs::read_to_string("creds.json")?;
    let creds: Creds = serde_json::from_str(&file)?;
    // let pass_bytes = base64::decode(creds.password.as_bytes())?;

    let bu_password = Password::new()
        .with_prompt("Enter Backup password")
        .interact()?;

    let mc = new_magic_crypt!(&bu_password, 256);

    let decrypt_string = mc
        .decrypt_base64_to_string(&creds.password)
        .with_context(|| format!("Wrong password!"))?;

    let creds_extended = CredsExtended {
        backup_password: bu_password,
        grant_type: creds.grant_type,
        username: creds.username,
        url: creds.url,
        password: decrypt_string,
    };

    Ok(creds_extended)
}

pub fn create_creds() -> Result<()> {
    let username: String = Input::new().with_prompt("Username").interact_text()?;

    let url: String = Input::new().with_prompt("Address").interact_text()?;

    let password = Password::new()
        .with_prompt("Enter VB365 password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?;

    let bu_password = Password::new()
        .with_prompt("Enter Backup password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?;

    let mc = new_magic_crypt!(bu_password, 256);
    let base64 = mc.encrypt_str_to_base64(password);

    let creds = Creds {
        grant_type: "password".to_string(),
        username,
        password: base64,
        url,
    };

    let mut file1 = File::create("creds.json")?;
    let string1 = serde_json::to_string(&creds)?;
    file1.write_all(string1.as_bytes())?;
    println!("Done");

    Ok(())
}
