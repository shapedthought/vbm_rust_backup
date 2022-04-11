# VBM Backup 

Exports VBM job configuration to json files. It's a compiled version of the Python version I created before using RUST.

It currently doesn't have a restore option or any command line flags. 

Also, due to the API two files are created, one with and the other with "selected items". 

Requires a creds.json file:

    {
    "grant_type": "password",
    "username": "administrator@domain.com",
    "password": "cGFzc3dvcmQK",
    "url": "192.168.0.123"
    }

The password needs to be in base64 for a little bit of passive security. 

This project is essentially some fun while learning async rust, and the reqwest package. 

Build from source:

    cargo build --release

Compiled program will be under target/release

