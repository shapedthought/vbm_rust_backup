use crate::getcreds::{create_creds, get_creds};
use crate::models::credsmodel::CredsResponse;
use crate::models::jobsmodel::{BackupJobSave, BackupJobs};
use crate::models::othermodels::OrgData;
use anyhow::Result;
use chrono;
use dialoguer::Confirm;
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde_json::Value;
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::Write;

pub async fn get_backups() -> Result<()> {
    if std::path::Path::new("creds.json").exists() == false {
        if Confirm::new()
            .with_prompt("No creds.json file, create?")
            .interact()?
        {
            create_creds()
        } else {
            println!("Exiting...");
            std::process::exit(1);
        }
    }

    let sp = Spinner::new(Spinners::Dots9, "Running Backup...".into());

    let creds = get_creds()?;

    let creds_urlenc = serde_urlencoded::to_string(&creds)?;

    let url = format!("https://{}:4443/v6/Token", creds.url);

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res_json: CredsResponse = post_data(&client, &headers, &url, creds_urlenc).await?;

    let bearer = format!("Bearer {}", res_json.access_token);
    let mut req_header = HeaderMap::new();
    req_header.insert(ACCEPT, "application/json".parse()?);
    req_header.insert(CONTENT_TYPE, "application/json".parse()?);
    req_header.insert("Authorization", bearer.parse()?);

    let org_url = format!("https://{}:4443/v6/Organizations/", creds.url);
    let org_data: Vec<OrgData> = get_data(&client, &req_header, &org_url).await?;

    let mut jobs = Vec::new();
    for item in org_data.iter() {
        let url = format!(
            "https://{}:4443/v5/organizations/{}/Jobs",
            creds.url, item.id
        );
        let job: Vec<BackupJobs> = get_data(&client, &req_header, &url).await?;

        jobs.push(job)
    }

    let mut select_jobs = Vec::new();
    // let mut non_select_jobs = Vec::new();
    for i in jobs.iter() {
        for j in i.iter() {
            let mut select_items: Option<Vec<Value>> = None;
            if j.backup_type == "SelectedItems" {
                let select_url = &j.links.selected_items.href;
                select_items = Some(get_data(&client, &req_header, &select_url).await.unwrap());
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
            };
            select_jobs.push(new_job)
        }
    }

    let date_time = chrono::offset::Local::now()
        .to_string()
        .replace(":", "-")
        .replace(" ", "_");

    let data_str: Vec<&str> = date_time.split(".").collect();

    let select_string = format!("jobs_backup_{}.json", data_str[0]);

    let mut file1 = File::create(select_string)?;
    let string1 = serde_json::to_string(&select_jobs)?;
    file1.write_all(string1.as_bytes())?;

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
