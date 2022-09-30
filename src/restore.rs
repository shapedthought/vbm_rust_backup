use anyhow::Result;
use std::fs;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{modifiers::UTF8_SOLID_INNER_BORDERS, Table};

pub async fn do_restores() -> Result<()> {
    let paths = fs::read_dir(".")?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_header(vec!["Index", "Name"]);

    let mut json_files = Vec::new();

    for i in paths {
        let path = i.unwrap().path().to_str().unwrap().to_string();

        if path.contains("select_jobs") {
            json_files.push(path);
        }
    }

    for (i, v) in json_files.iter().enumerate() {

        let str = v.to_owned();

        table.add_row(vec![i.to_string(), str]);
    }

    println!("\nSelect which file you want to Restore from");
    println!("{table}");

    Ok(())
}