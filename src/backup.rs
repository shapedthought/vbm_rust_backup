use crate::getcreds::{create_creds, get_creds};
use crate::models::credsmodel::{Creds, CredsResponse};
use crate::models::jobsmodel::{BackupJobSave, BackupJobs};
use crate::models::othermodels::OrgData;
use anyhow::Result;
use dialoguer::Confirm;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde_json::Value;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::Write;

pub async fn get_backups(pass_env: bool) -> Result<()> {
    if !std::path::Path::new("creds.json").exists() {
        if Confirm::new()
            .with_prompt("No creds.json file, create?")
            .interact()?
        {
            create_creds(None)?;
        } else {
            println!("Exiting...");
            std::process::exit(1);
        }
    }

    let creds = get_creds(pass_env)?;

    let send_creds = Creds {
        grant_type: creds.grant_type,
        username: creds.username,
        password: creds.password,
        url: creds.url,
    };

    let mut sp = Spinner::new(Spinners::Dots9, "Running Backup...".into());

    let creds_urlenc = serde_urlencoded::to_string(&send_creds)?;

    let url = format!(
        "https://{}:{}/{}/Token",
        send_creds.url, creds.port, creds.api_version
    );

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(creds.insecure)
        .build()?;

    let res_json: CredsResponse = post_data(&client, &headers, &url, creds_urlenc).await?;

    let bearer = format!("Bearer {}", res_json.access_token);
    let mut req_header = HeaderMap::new();
    req_header.insert(ACCEPT, "application/json".parse()?);
    req_header.insert(CONTENT_TYPE, "application/json".parse()?);
    req_header.insert("Authorization", bearer.parse()?);

    let org_url = format!(
        "https://{}:{}/{}/Organizations/",
        send_creds.url, creds.port, creds.api_version
    );
    let org_data: Vec<OrgData> = get_data(&client, &req_header, &org_url).await?;

    let mut jobs = Vec::new();
    for item in org_data.iter() {
        let url = format!(
            "https://{}:{}/{}/organizations/{}/Jobs",
            send_creds.url, creds.port, creds.api_version, item.id
        );
        let job: Vec<BackupJobs> = get_data(&client, &req_header, &url).await?;

        jobs.push(job)
    }

    let mut select_jobs = Vec::new();
    for i in jobs.iter() {
        for j in i.iter() {
            let mut select_items: Option<Vec<Value>> = None;
            let mut excluded_items: Option<Vec<Value>> = None;
            let mut version_number = 6;

            if j.backup_type == "SelectedItems" {
                let href = &j.links.selected_items.as_ref().unwrap().href;

                version_number = creds.api_version.split_at(1).1.parse::<u8>()?;

                let select_url: String = if version_number > 5 {
                    format!("https://{}:{}/{}", send_creds.url, creds.port, href)
                } else {
                    href.to_string()
                };

                let data = get_data(&client, &req_header, &select_url).await?;
                select_items = Some(data);
            };
            if let Some(exc_url) = &j.links.excluded_items {

                let excluded_url: String = if version_number > 5 {
                    format!("https://{}:{}/{}", send_creds.url, creds.port, exc_url.href)
                } else {
                    exc_url.href.to_string()
                };

                let ex_data: Vec<Value> = get_data(&client, &req_header, &excluded_url).await?;
                excluded_items = Some(ex_data)
            }
            let new_job = BackupJobSave {
                backup_type: j.backup_type.to_string(),
                selected_items: select_items,
                description: j.description.to_string(),
                id: j.id.to_string(),
                is_enabled: j.is_enabled,
                name: j.name.to_string(),
                repository_id: j.repository_id.to_string(),
                schedule_policy: j.schedule_policy.clone(),
                run_now: false,
                excluded_items: excluded_items
            };
            select_jobs.push(new_job)
        }
    }

    let date_time = chrono::offset::Local::now()
        .to_string()
        .replace(':', "-")
        .replace(' ', "_");

    let data_str: Vec<&str> = date_time.split('.').collect();

    let select_string = format!("jobs_backup_{}", data_str[0]);

    let extended_password = format!("{}:{}", creds.backup_password, send_creds.password);

    let encrypt_password = base64::encode(extended_password.as_bytes());

    let mc = new_magic_crypt!(encrypt_password, 256);

    let mut file1 = File::create(select_string)?;
    let string1 = serde_json::to_string(&select_jobs)?;

    let base64 = mc.encrypt_str_to_base64(string1);

    file1.write_all(base64.as_bytes())?;

    sp.stop();
    println!("\nComplete!");

    Ok(())
}

pub async fn get_data<T: DeserializeOwned>(
    client: &reqwest::Client,
    headers: &HeaderMap,
    url: &str,
) -> Result<T> {
    let data = client
        .get(url)
        .headers(headers.clone())
        .send()
        .await?
        .json()
        .await?;

    Ok(data)
}

pub async fn post_data<T: DeserializeOwned>(
    client: &reqwest::Client,
    headers: &HeaderMap,
    url: &str,
    data: String,
) -> Result<T> {
    let res_data = client
        .post(url)
        .body(data)
        .headers(headers.clone())
        .send()
        .await?
        .json()
        .await?;

    Ok(res_data)
}
