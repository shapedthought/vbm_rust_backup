use jobs::{BackupJobs, BackupJobSave};
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use anyhow::Result;
// use spinners::{Spinner, Spinners};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Response {
    access_token: String,
    token_type: String,
    refresh_token: String,
    expires_in: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Creds {
    grant_type: String,
    username: String,
    password: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct OrgData {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let file = fs::read_to_string("creds.json").unwrap();
    let mut creds: Creds = serde_json::from_str(&file).unwrap();
    let pass_bytes = base64::decode(creds.password.as_bytes()).unwrap();
    let password = String::from_utf8_lossy(&pass_bytes);

    creds.password = password.to_string();

    let creds_urlenc = serde_urlencoded::to_string(&creds)?;

    let url = format!("https://{}:4443/v6/Token", creds.url);

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    // headers.insert("x-api-version", "1.0-rev2".parse()?);
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res_json: Response = post_data(&client, &headers, &url, creds_urlenc).await?;

    let bearer = format!("Bearer {}", res_json.access_token);
    let mut req_header = HeaderMap::new();
    req_header.insert(ACCEPT, "application/json".parse()?);
    req_header.insert(CONTENT_TYPE, "application/json".parse()?);
    // req_header.insert("grant_type", "token".parse()?);
    req_header.insert("Authorization", bearer.parse()?);

    let org_url = format!("https://{}:4443/v6/Organizations/", creds.url);
    let org_data: Vec<OrgData> = get_data(&client, &req_header, &org_url).await?;

    println!("Name: {:?}: id: {:?}", org_data[0].name, org_data[0].id);
    // write_file("token.json", res_json).expect("Someting went wrong");

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
    let mut non_select_jobs = Vec::new();
    for i in jobs.iter() {
        for j in i.iter() {
            if j.backup_type == "SelectedItems" {
                let select_url = &j.links.selected_items.href;
                let select_items: Value = get_data(&client, &req_header, &select_url).await.unwrap();

                let new_job = BackupJobSave {
                    backup_type: select_items,
                    description: j.description.to_string(),
                    id: j.id.to_string(),
                    is_enabled: j.is_enabled,
                    name: j.name.to_string(),
                    repository_id: j.repository_id.to_string(),
                    schedule_policy: j.schedule_policy.clone()
                };
                select_jobs.push(new_job);
            } else {
                non_select_jobs.push(j);
            }
        }
    }

    let mut file1 = File:: create("select_jobs.json")?;
    let string1 = serde_json::to_string(&select_jobs)?;
    file1.write_all(string1.as_bytes())?;

    let mut file2 = File::create("non_select_jobs.json")?;
    let string2 = serde_json::to_string(&non_select_jobs)?;
    file2.write_all(string2.as_bytes())?;

    Ok(())
}

async fn get_data<T: DeserializeOwned>(
    client: &reqwest::Client, 
    headers: &HeaderMap, 
    url: &str) -> Result<T>{
    let data = client
        .get(url)
        .headers(headers.clone())
        .send()
        .await?
        .json()
        .await?;

    Ok(data)
}

async fn post_data<T: DeserializeOwned>(
    client: &reqwest::Client,
    headers: &HeaderMap,
    url: &str,
    data: String
    )
    -> Result<T> {
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