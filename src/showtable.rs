use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
// use comfy_table::ContentArrangement;
use comfy_table::{modifiers::UTF8_SOLID_INNER_BORDERS, Table};
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::fs;

use crate::getcreds::get_creds;
use crate::models::jobsmodel::BackupJobSave;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn print_table() {
    let creds = get_creds().unwrap();

    let paths = fs::read_dir(".").unwrap();

    let mut json_files = Vec::new();

    for i in paths {
        let path = i.unwrap().path().to_str().unwrap().to_string();

        if path.contains("job") {
            json_files.push(path);
        }
    }

    let mut file_strings = Vec::new();
    for (i, v) in json_files.iter().enumerate() {
        let str = v.to_owned();
        let job_string = format!("{}. {}", i, str);

        file_strings.push(job_string);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select job to print")
        .items(&file_strings)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    let selected_file = &json_files[selection];
    let file = fs::read_to_string(selected_file).unwrap();

    let extended_password = format!("{}:{}", creds.backup_password, creds.password);

    let encrypt_password = base64::encode(extended_password.as_bytes());

    let mc = new_magic_crypt!(encrypt_password, 256);

    let decrypted_string = mc.decrypt_base64_to_string(file).unwrap();

    let backuped_jobs: Vec<BackupJobSave> = serde_json::from_str(&decrypted_string).unwrap();

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_header(vec![
            "Name",
            "Backup Type",
            "Description",
            "Repo ID",
            "Is Enabled",
            "Schedule Type",
            "Schedule Time",
        ]);

    for i in backuped_jobs.iter() {
        let enabled_str = if i.is_enabled {
            "true".to_string()
        } else {
            "false".to_string()
        };

        let daily_type = &i.schedule_policy.daily_type;
        let daily_time = &i.schedule_policy.daily_time;

        table.add_row(vec![
            i.name.to_string(),
            i.backup_type.to_string(),
            i.description.to_string(),
            i.repository_id.to_string(),
            enabled_str,
            daily_type.to_string(),
            daily_time.to_string(),
            // st_strings
        ]);
    }

    println!("{table}")
}
