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

For all operations a creds.json file is required. 

    {
        "grant_type": "password",
        "username": "administrator@domain.com",
        "password": "cGFzc3dvcmQK",
        "url": "192.168.0.123"
    }

The password is for VB365 but is encrypted with a backup password which you enter when you create the file using
the --creds flag.

Having the password in the creds file be encrypted means that non-admin users to use the tool without providing them with the credentials of VB365.

## Backup

To run a backup just run without any flags, it will prompt for the backup password, if correct it will run the job backup.

The backup file is also encrypted with AES256 using a password which is a combination of both the backup and VB365 passwords.

## Restore

Using the -r / --restore flag the tool will take you through a wizard:

1. Select the file you want to restore from
    - The tool will look for files in the directory with "job" in the name
2. Enter the backup password
3. Select the job you want to restore
4. Select the Org to restore to
5. Select the Proxy you want to use
6. Select the Repo you want to use
7. Confirm the restore

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

