use anyhow::Result;
// use dialoguer::console::Term;
// use dialoguer::{theme::ColorfulTheme, Select};
use clap::Parser;

mod backup;
use backup::get_backups;
mod restore;
use getcreds::create_creds;
use restore::do_restores;
pub mod models;
pub mod getcreds;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Runs the restore action
    #[arg(short, long)]
    restore: bool,

    /// Create creds.json file
    #[arg(short, long)]
    creds: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // println!("Selection which option you require");
    let cli = Cli::parse();

    if cli.creds {
        create_creds();
        std::process::exit(1);
    }

    if cli.restore {
        do_restores().await?;
    } else {
        get_backups().await?;
    }
    Ok(())
}
