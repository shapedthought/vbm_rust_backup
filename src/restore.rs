use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{modifiers::UTF8_SOLID_INNER_BORDERS, Table};
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use std::fs;
use crate::getcreds::get_creds;
use crate::models::credsmodel::CredsResponse;
use crate::models::jobsmodel::BackupJobSave;
use crate::models::othermodels::OrgData;
use crate::models::repomodel::RepoModel;

use crate::models::servermodels::ProxyModel;
// use serde_json::Value;

#[derive(Debug)]
#[allow(dead_code)]
struct ProxyRepo {
    proxy_id: String,
    proxy_name: String,
    repos: Vec<RepoDetails>
}

#[derive(Debug)]
#[allow(dead_code)]
struct RepoDetails {
    repo_name: String,
    repo_id: String,
    is_long_term: Option<bool>
}

fn print_table(json_files: &Vec<String>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_header(vec!["Index", "Name"]);

    for (i, v) in json_files.iter().enumerate() {
        let str = v.to_owned();

        table.add_row(vec![i.to_string(), str]);
    }

    println!("\nSelect which file you want to Restore from");
    println!("{table}");
}

pub async fn run_restores(file_name: &String) -> Result<()> {
    let creds = get_creds().unwrap();

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

    // get orgnainisations

    let org_url = format!("https://{}:4443/v6/Organizations/", creds.url);
    let org_data: Vec<OrgData> = get_data(&client, &req_header, &org_url).await?;
    let org_names: Vec<String> = org_data.iter().map(|x| x.name.clone()).collect();

    // get proxies
    let proxy_url = format!("https://{}:4443/v6/Proxies?extendedView=true/", creds.url);
    let proxy_data: Vec<ProxyModel> = get_data(&client, &req_header, &proxy_url).await?;

    let mut repos: Vec<ProxyRepo> = Vec::new();
    for i in proxy_data {
        let repo_url = format!("https://{}:4443/{}", creds.url, i.links.repositories.href);
        let repo_data: Vec<RepoModel> = get_data(&client, &req_header, &repo_url).await?;

        let repo_details: Vec<RepoDetails> = repo_data.iter().map(|x| RepoDetails {
            repo_id: x.id.clone(),
            repo_name: x.name.clone(),
            is_long_term: x.is_long_term
        }).collect();

        let proxy_repo = ProxyRepo {
            proxy_id: i.id,
            proxy_name: i.host_name,
            repos: repo_details
        };
        repos.push(proxy_repo);
    }
    let proxy_names: Vec<String> = repos.iter().map(|x| x.proxy_name.clone()).collect();

    // Read the jobs file
    let file = fs::read_to_string(file_name)?;
    let mut backuped_jobs: Vec<BackupJobSave> = serde_json::from_str(&file)?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_header(vec!["Index", "Name", "Description"]);

    for (i, v) in backuped_jobs.iter().enumerate() {
        let name = v.name.to_owned();
        let description = v.description.to_owned();

        table.add_row(vec![i.to_string(), name, description]);
    }

    println!("{table}");
    let options = (0..backuped_jobs.len())
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            // println!("Selected item: {}", index);
            let mut job = &mut backuped_jobs[index];
            println!("{:} {:}", job.name, job.backup_type);

            // select org
            println!("Select Org to restore to:");
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&org_names)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            let mut org_id: String = String::new();
            if let Some(i) = selection {
                println!("ID: {:?}, Name: {:?}", org_data[i].id, org_data[i].name);
                org_id = org_data[i].id.clone();
            }

            // select proxy
            println!("Select Proxy:");
            let proxy_select = Select::with_theme(&ColorfulTheme::default())
                .items(&proxy_names)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            println!("Select Repository:");
            if let Some(i) = proxy_select {
                let repo_names: Vec<String> = repos[i].repos.iter().map(|x| x.repo_name.clone()).collect();
                let repo_select = Select::with_theme(&ColorfulTheme::default())
                    .items(&repo_names)
                    .default(0)
                    .interact_on_opt(&Term::stderr())?;

                if let Some(j) = repo_select {
                    job.repository_id = repos[i].repos[j].repo_id.clone();

                    // https://{URL}:4443/v6/Organizations/{id}/Jobs
                    let job_url = format!("https://{}:4443/v6/Organizations/{}/Jobs", creds.url, org_id);
                    println!("{:}", job_url);
                    let res = client.post(job_url)
                        .headers(req_header)
                        .json(&job)
                        .send()
                        .await?;

                    println!("{:#?}", res);

                }
            }
        }
        None => println!("Nothing selected"),
    }
 

    // confirm restore

    // post backups to the API
    Ok(())
}

pub async fn do_restores() -> Result<()> {
    let paths = fs::read_dir(".")?;

    let mut json_files = Vec::new();

    for i in paths {
        let path = i.unwrap().path().to_str().unwrap().to_string();

        if path.contains("jobs") {
            json_files.push(path);
        }
    }

    match json_files.len() {
        1 => {
            println!("File found: {}", json_files[0]);
            if Confirm::new()
                .with_prompt("Do you want to continue?")
                .interact()?
            {
                println!("Looks like you want to continue");
                run_restores(&json_files[1]).await?;
            } else {
                println!("nevermind then :(");
            }
        }
        2.. => {
            print_table(&json_files);

            let options = (0..json_files.len())
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&options)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(index) => {
                    // println!("Selected item: {}", index);
                    let file = &json_files[index];
                    run_restores(file).await?;
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