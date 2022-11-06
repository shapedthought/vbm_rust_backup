use assert_cmd::Command;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadCreds {
    pub grant_type: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub port: u16,
    pub api_version: String,
}

#[test]
fn dies_wrong_args() {
    let mut cmd = Command::cargo_bin("vbm_rust_backup").unwrap();
    cmd.arg("-e").assert().failure();
}

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("vbm_rust_backup").unwrap();
    cmd.arg("").assert().failure();
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("vbm_rust_backup").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn create_creds() {
    let mut cmd = Command::cargo_bin("vbm_rust_backup").unwrap();
    cmd.arg("creds-ni")
        .arg("-u administrator")
        .arg("-a 192.168.0.123")
        .arg("-v password")
        .arg("-b password")
        .assert()
        .success();
}

#[test]
fn check_password() {
    let first_file = fs::read_to_string("/Users/edwardhoward/Documents/RUST/vbm_backup/creds.json").unwrap();
    let first_creds: ReadCreds = serde_json::from_str(&first_file).unwrap();

    let second_file = fs::read_to_string("/Users/edwardhoward/Documents/RUST/vbm_backup/tests/creds_test.json").unwrap();
    let second_creds: ReadCreds = serde_json::from_str(&second_file).unwrap();

    assert_eq!(first_creds.password, second_creds.password)
}