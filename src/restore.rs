use crate::getcreds::{create_creds, get_creds};
use crate::models::credsmodel::{Creds, CredsExtended, CredsResponse};
use crate::models::jobsmodel::{BackupJobSave, BackupJobs};
use crate::models::othermodels::OrgData;
use crate::models::repomodel::RepoModel;
use anyhow::Result;
use colored::*;
use dialoguer::console::Term;
use dialoguer::MultiSelect;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use std::fs;

use crate::models::servermodels::ProxyModel;

#[derive(Debug)]
#[allow(dead_code)]
struct ProxyRepo {
    proxy_id: String,
    proxy_name: String,
    repos: Vec<RepoDetails>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct RepoDetails {
    repo_name: String,
    repo_id: String,
    is_long_term: Option<bool>,
}

fn make_selection(text: &str, selections: &[String]) -> Result<Option<usize>, std::io::Error> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(text)
        .items(selections)
        .default(0)
        .interact_on_opt(&Term::stderr())
}

pub async fn run_restores(file_name: &String, creds: &CredsExtended) -> Result<()> {
    // need to do some work to clean this bit up
    let send_creds = Creds::new(
        creds.grant_type.clone(),
        creds.username.clone(),
        creds.password.clone(),
        creds.url.clone(),
    );

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

    // get organizations
    let org_url = format!(
        "https://{}:{}/{}/Organizations/",
        send_creds.url, creds.port, creds.api_version
    );
    let org_data: Vec<OrgData> = get_data(&client, &req_header, &org_url).await?;
    let org_names: Vec<String> = org_data.iter().map(|x| x.name.clone()).collect();

    // get current jobs
    let jobs_url = format!(
        "https://{}:{}/{}/Jobs",
        send_creds.url, creds.port, creds.api_version
    );

    let jobs: Vec<BackupJobs> = get_data(&client, &req_header, &jobs_url).await?;
    let current_jobs_str: Vec<String> = jobs.iter().map(|x| x.name.clone()).collect();
    let job_types = jobs
        .iter()
        .filter(|x| x.backup_type.contains("EntireOrganization"));

    let any_org = job_types.count();

    // get proxies
    let proxy_url = format!(
        "https://{}:{}/{}/Proxies?extendedView=true/",
        send_creds.url, creds.port, creds.api_version
    );
    let proxy_data: Vec<ProxyModel> = get_data(&client, &req_header, &proxy_url).await?;

    let mut repos: Vec<ProxyRepo> = Vec::new();
    for i in proxy_data {
        let repo_url = format!(
            "https://{}:{}/{}",
            send_creds.url, creds.port, i.links.repositories.href
        );
        let repo_data: Vec<RepoModel> = get_data(&client, &req_header, &repo_url).await?;

        let repo_details: Vec<RepoDetails> = repo_data
            .iter()
            .map(|x| RepoDetails {
                repo_id: x.id.clone(),
                repo_name: x.name.clone(),
                is_long_term: x.is_long_term,
            })
            .collect();

        let proxy_repo = ProxyRepo {
            proxy_id: i.id,
            proxy_name: i.host_name,
            repos: repo_details,
        };
        repos.push(proxy_repo);
    }
    let proxy_names: Vec<String> = repos.iter().map(|x| x.proxy_name.clone()).collect();

    // Read the jobs file
    let file = fs::read_to_string(file_name)?;

    let extended_password = format!("{}:{}", creds.backup_password, send_creds.password);

    let encrypt_password = base64::encode(extended_password.as_bytes());

    let mc = new_magic_crypt!(encrypt_password, 256);

    let decrypt_string = mc.decrypt_base64_to_string(&file)?;

    let mut backuped_jobs: Vec<BackupJobSave> = serde_json::from_str(&decrypt_string)?;

    let filtered_jobs: Vec<&BackupJobSave> = backuped_jobs
        .iter()
        .filter(|x| {
            if any_org > 0 {
                x.backup_type != "EntireOrganization"
            } else {
                !x.backup_type.is_empty()
            }
        })
        .collect();

    let mut job_strings = Vec::new();

    for (i, v) in filtered_jobs.iter().enumerate() {
        let name = v.name.to_owned();

        let job_string = format!("{}. {}", i, name);
        job_strings.push(job_string);
    }

    let chosen: Vec<usize> = MultiSelect::new()
        .items(&job_strings)
        .report(true)
        .with_prompt("Select Backups to Restore")
        .interact()?;

    for i in chosen {
        let mut job = &mut backuped_jobs[i];

        if current_jobs_str.contains(&job.name) {
            println!("{}", "Job Name in use, appending with - restored".cyan());
            job.name.push_str("- restored");
        }

        println!("{} Options", job.name);

        // select org
        let selection = make_selection("Select Org to restore to", &org_names)?;

        let mut org_id: String = String::new();
        if let Some(i) = selection {
            org_id = org_data[i].id.clone();
        }

        // select proxy
        let proxy_select = make_selection("Select Proxy", &proxy_names)?;

        if let Some(i) = proxy_select {
            let repo_names: Vec<String> =
                repos[i].repos.iter().map(|x| x.repo_name.clone()).collect();

            let repo_select = make_selection("Select Repository", &repo_names)?;

            if let Some(j) = repo_select {
                job.repository_id = repos[i].repos[j].repo_id.clone();

                if Confirm::new().with_prompt("Restore?").interact()? {
                    let job_url = format!(
                        "https://{}:{}/{}/Organizations/{}/Jobs",
                        send_creds.url, creds.port, creds.api_version, org_id
                    );
                    let res = client
                        .post(job_url)
                        .headers(req_header.clone())
                        .json(&job)
                        .send()
                        .await?;

                    if res.status().is_success() {
                        println!("{}", "success!".green())
                    } else {
                        println!("{}", "error!".red());
                        println!("{:?}", res)
                    }
                } else {
                    println!("Cancelled..");
                }
            }
        }
    }

    Ok(())
}

pub async fn do_restores() -> Result<()> {
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
    let paths = fs::read_dir(".")?;

    let mut json_files = Vec::new();

    for i in paths {
        let path = i?.path().to_str().unwrap().to_string();

        if path.contains("job") {
            json_files.push(path);
        }
    }

    let creds = get_creds()?;

    match json_files.len() {
        1 => {
            println!("File found: {}", json_files[0]);
            if Confirm::new()
                .with_prompt("Do you want to continue?")
                .interact()?
            {
                loop {
                    run_restores(&json_files[0], &creds).await?;
                    if Confirm::new().with_prompt("Run another?").interact()? {
                        continue;
                    } else {
                        break;
                    }
                }
            } else {
                println!("nevermind then :(");
            }
        }
        2.. => {
            let mut file_strings = Vec::new();
            for (i, v) in json_files.iter().enumerate() {
                let str = v.to_owned();
                let job_string = format!("{}. {}", i, str);

                file_strings.push(job_string);
            }

            let selection = make_selection("Select Job file to restore from", &file_strings)?;

            match selection {
                Some(index) => {
                    let file = &json_files[index];
                    loop {
                        run_restores(file, &creds).await?;
                        if Confirm::new().with_prompt("Run another?").interact()? {
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                None => println!("Nothing selected"),
            }
        }
        _ => println!("No files found!!!"),
    }
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
