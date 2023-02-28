use anyhow::Result;
use clap::{Parser, Subcommand};

mod backup;
use backup::get_backups;
use colored::Colorize;
use dialoguer::Confirm;
mod restore;
use crate::models::credsmodel::CredsExtended;
use getcreds::create_creds;
use restore::do_restores;
use showtable::print_table;
pub mod getcreds;
pub mod models;
pub mod showtable;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Runs the restore action
    #[arg(short, long)]
    restore: bool,

    /// Create a creds.json file interactively
    #[arg(short, long)]
    creds: bool,

    /// Print the info about a backup file
    #[arg(short, long)]
    table: bool,

    /// Use VB365_PASS env variable for password on backup and restore
    #[arg(short, long)]
    env_pass: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, PartialEq)]
enum Commands {
    /// Create creds file non-interactively
    CredsNI {
        /// vb365 username
        #[arg(short, long)]
        username: String,

        ///vb365 address
        #[arg(short, long)]
        address: String,

        /// vb365 password
        #[arg(short, long)]
        vb365_password: String,

        /// backup password
        #[arg(short, long)]
        backup_password: String,

        /// vb365 port
        #[arg(short, long, default_value_t = 4443)]
        port: u16,

        /// vb365 version
        #[arg(long, default_value_t = String::from("v7"))]
        api_version: String,

        /// Allow Invalid Certificates
        #[arg(short, long)]
        insecure: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.table {
        print_table(cli.env_pass)?;
    }

    if cli.creds {
        println!(
            "{}",
            "The VB365 Job Backup Tool is supplied without warranty or support.".green()
        );
        if Confirm::new()
            .with_prompt("Confirm you understand?")
            .interact()?
        {
            println!("{}", "Confirmed".green());
            create_creds(None)?;
        } else {
            eprint!("{}", "Exiting...".red())
        }
    }

    if cli.restore {
        do_restores(cli.env_pass).await?;
    } else if Option::is_none(&cli.command) && !cli.creds && !cli.table {
        get_backups(cli.env_pass).await?;
    } else if let Some(Commands::CredsNI {
        username,
        address,
        vb365_password,
        backup_password,
        port,
        api_version,
        insecure,
    }) = cli.command
    {
        let grant_type = String::from("password");
        let read_creds = CredsExtended {
            backup_password,
            username,
            grant_type,
            password: vb365_password,
            url: address,
            port,
            api_version,
            insecure,
        };
        create_creds(Some(read_creds))?;
    }

    Ok(())
}
