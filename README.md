# VBM Job Backup 

This tool backs up and restores Veeam Backup for M365 jobs. 

Currently only restores individual jobs.

CLI commands:

    Usage: vbm_rust_backup [OPTIONS]

    Options:
        -r, --restore  Runs the restore action
        -c, --creds    Create a creds.json file
        -t, --table    Print the info in a backup file
        -h, --help     Print help information
        -V, --version  Print version information

By default the tool will backup as long as the creds.json file is present. 

The output is encrypted with AES256 using the password supplied in the creds file.

If the tool will ask if you want to create the file and take you through a wizard.

creds.json file: 

    {
        "grant_type": "password",
        "username": "administrator@domain.com",
        "password": "cGFzc3dvcmQK",
        "url": "192.168.0.123"
    }

The password needs to be in base64 for a little bit of passive security. 

Using the -r / --restore flag the tool will take you through a wizard:

1. Select the file you want to restore from
    - The tool will look for json files in the directory with "job" in the name
2. Select the job you want to restore
3. Select the Org to restore to
4. Select the Proxy you want to use
5. Select the Repo you want to use
6. Confirm the restore

## How to set up

1. To use install RUST https://www.rust-lang.org/tools/install 
2. Clone this repo
3. Open terminal in the root directory
3. Build

Build command:

    cargo build --release

Compiled program will be under target/release

Run the tool via a terminal 

    .\vbm_backup.exe

