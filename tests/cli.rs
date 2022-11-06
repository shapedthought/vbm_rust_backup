use assert_cmd::Command;
use serde::{Deserialize, Serialize};
use std::{env, fs};

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

struct PathStrings {
    first_string: String,
    second_string: String
}

fn get_paths()-> PathStrings {
    let binding = env::current_dir().unwrap();
    let path = binding.to_str().unwrap();

    let split_char: String;

    if cfg!(windows) {
        split_char = "\\".to_string()
    } else {
        split_char = "/".to_string()
    }

    let first_string = format!("{path}{split_char}creds.json");
    let second_string = format!("{path}{split_char}tests{split_char}creds_test.json");

    PathStrings {
        first_string,
        second_string
    }
}

#[test]
fn check_password() {
    let path_strings: PathStrings = get_paths();

    let first_string = path_strings.first_string;
    let second_string = path_strings.second_string;

    let first_file = fs::read_to_string(first_string).unwrap();
    let first_creds: ReadCreds = serde_json::from_str(&first_file).unwrap();

    let second_file = fs::read_to_string(second_string).unwrap();
    let second_creds: ReadCreds = serde_json::from_str(&second_file).unwrap();

    assert_eq!(first_creds.password, second_creds.password)
}
