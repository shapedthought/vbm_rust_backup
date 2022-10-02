use anyhow::Result;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

mod backup;
use backup::get_backups;
mod restore;
use restore::do_restores;
pub mod models;
pub mod getcreds;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Selection which option you require");
    let items = vec!["Backup", "Restore"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if index == 0 {
                get_backups().await?;
            } else {
                do_restores().await?;
            }
        }
        None => println!("User did not select anything"),
    }

    Ok(())
}
