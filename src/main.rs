use jobs::{BackupJobs, BackupJobSave};
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // let sp = Spinner::new(Spinners::Dots9, "Getting data".into());
    let resp = client
        .post(url)
        .body(creds_urlenc)
        .headers(headers)
        .send()
        .await?;

    println!("Status code: {:?}", resp.status());

    // sp.stop();

    let res_json = resp.json::<Response>().await?;

    let bearer = format!("Bearer {}", res_json.access_token);
    let mut req_header = HeaderMap::new();
    req_header.insert(ACCEPT, "application/json".parse()?);
    req_header.insert(CONTENT_TYPE, "application/json".parse()?);
    // req_header.insert("grant_type", "token".parse()?);
    req_header.insert("Authorization", bearer.parse()?);

    let org_url = format!("https://{}:4443/v6/Organizations/", creds.url);
    let org_data: Vec<OrgData> = client
        .get(org_url)
        .headers(req_header.clone())
        .send()
        .await?
        .json()
        .await?;

    println!("Name: {:?}: id: {:?}", org_data[0].name, org_data[0].id);
    // write_file("token.json", res_json).expect("Someting went wrong");

    let mut jobs = Vec::new();
    for item in org_data.iter() {
        let url = format!(
            "https://{}:4443/v5/organizations/{}/Jobs",
            creds.url, item.id
        );
        let job: Vec<BackupJobs> = client
            .get(url)
            .headers(req_header.clone())
            .send()
            .await?
            .json()
            .await?;

        jobs.push(job)
    }

    let mut select_jobs = Vec::new();
    let mut non_select_jobs = Vec::new();
    for i in jobs.iter() {
        for j in i.iter() {
            if j.backup_type == "SelectedItems" {
                let select_url = &j.links.selected_items.href;
                let select_items: Value = client
                    .get(select_url)
                    .headers(req_header.clone())
                    .send()
                    .await?
                    .json()
                    .await?;
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

// #[allow(dead_code)]
// fn write_file<U, T>(name: U, data: T) -> std::io::Result<()> {
//     let mut file = File::create(name)?;
//     let strings = serde_json::to_string(&data)?;
//     file.write_all(strings.as_bytes())?;
//     Ok(())
// }

//echo -n | openssl s_client -showcerts -servername vi1.virtausyntheic.co.uk -connect vi1.virtausyntheic.co.uk:9419 | openssl x509 > api.cert
