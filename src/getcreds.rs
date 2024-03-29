use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::{bail, Context, Result};
use base64;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Password, Select};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::env;

use crate::models::credsmodel::{CredsExtended, ReadCreds};

pub fn get_creds(pass_env: bool) -> Result<CredsExtended> {
    let path_env = env::var("VB365_CONFIG");

    let file: String;

    if path_env.is_ok() {
        file = fs::read_to_string(path_env.unwrap())?;
    } else {
        file = fs::read_to_string("creds.json")?;
    }

    // let file = fs::read_to_string("creds.json")?;
    let creds: ReadCreds = serde_json::from_str(&file)?;

    let bu_password: String;

    if pass_env {
        match env::var("VB365_PASS") {
            Ok(pass) => bu_password = pass.to_string(),
            Err(_) => bail!("VB365_PASS not set in env"),
        }
        // let bu_password_opt: Option<&'static str> = option_env!("VB365_PASS");
        // bu_password = bu_password_opt
    } else {
        bu_password = Password::new()
            .with_prompt("Enter Backup password")
            .interact()?;
    }

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
        insecure: creds.insecure,
    };

    Ok(creds_extended)
}

pub fn create_creds(ni_creds: Option<CredsExtended>) -> Result<()> {
    let username: String;
    let url: String;
    let port: u16;
    let api_version: String;
    let password: String;
    let bu_password: String;
    let insecure: bool;

    match ni_creds {
        Some(ni_creds) => {
            username = ni_creds.username;
            url = ni_creds.url;
            port = ni_creds.port;
            api_version = ni_creds.api_version;
            password = ni_creds.password;
            bu_password = ni_creds.backup_password;
            insecure = ni_creds.insecure
        }
        None => {
            username = Input::new().with_prompt("Username").interact_text()?;

            url = Input::new().with_prompt("Address").interact_text()?;

            port = Input::new()
                .with_prompt("Select Port")
                .default(4443)
                .interact_text()?;

            api_version = Input::new()
                .with_prompt("Select API version")
                .default("v7".into())
                .interact_text()?;

            password = Password::new()
                .with_prompt("Enter VB365 password")
                .with_confirmation("Confirm password", "Passwords mismatching")
                .interact()?;

            bu_password = Password::new()
                .with_prompt("Enter Backup password")
                .with_confirmation("Confirm password", "Passwords mismatching")
                .interact()?;

            let options = vec!["no", "yes"];
            let select_insecure = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Allow Invalid Certificates?")
                .items(&options)
                .default(0)
                .interact_on_opt(&Term::stderr())
                .unwrap()
                .unwrap();

            if select_insecure == 0 {
                insecure = false
            } else {
                insecure = true
            }
        }
    }

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
        insecure,
    };

    let mut file1 = File::create("creds.json")?;
    let string1 = serde_json::to_string(&creds)?;
    file1.write_all(string1.as_bytes())?;
    println!("Done");

    Ok(())
}
