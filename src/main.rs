use anyhow::Result;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use dialoguer::console::Term;

mod backup;
use backup::get_backups;
mod restore;
use restore::do_restores;

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
        },
        None => println!("User did not select anything")
    }

    Ok(())
}
