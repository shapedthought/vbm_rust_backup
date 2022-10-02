# VBM Backup 

This tool backsup and restores Veeam Backup for M365 jobs. 

When you run the tool it will ask if you want to backup or restore. 

Both options require a creds.json file in the same directory where you are running the tool:

    {
    "grant_type": "password",
    "username": "administrator@domain.com",
    "password": "cGFzc3dvcmQK",
    "url": "192.168.0.123"
    }

The password needs to be in base64 for a little bit of passive security. 

Restores will ask a series of questions:

1. Select the file you want to restore from
2. Select the job you want to restore
3. Select the Org to restore to
4. Select the Proxy you want to use
5. Select the Repo you want to use

After the last step it will create the job.

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

