use core::str;
use std::process::Stdio;
use std::env::current_dir;

use anyhow::{anyhow, Ok};
use tokio::process::Command;

#[derive(Debug)]
pub struct Manager {
    path: String,
    command: Command
}

impl Manager {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            command: Command::new("adb")
        }
    }
    pub async fn check_for_adb(&mut self) -> Result<(), anyhow::Error> {
        let mut adb_dir = current_dir()
                    .expect("Failed to get current working directory");
        adb_dir.extend(&["platform-tools", "adb"]);
        let mut cmd_system = Command::new("adb");
        let mut cmd_local = Command::new(&adb_dir);

        if Manager::check_for_adb_file(&mut cmd_local).await.unwrap() {
            self.path = adb_dir.into_os_string().into_string().unwrap();
            self.command = cmd_local;
            Ok(())

        }
        else if Manager::check_for_adb_file(&mut cmd_system).await.unwrap() {
            self.path = "adb".to_string();
            self.command = cmd_system;
            Ok(())
        }
        else {
            Err(anyhow!("NAH"))

        }
    }

    async fn check_for_adb_file(cmd: &mut Command) -> Result<bool, anyhow::Error> {

        const ADB_VERSION_RESULT: &str = "Android Debug Bridge version";
        cmd.arg("version");

        cmd.stdout(Stdio::piped());

        let child= cmd.spawn();
        match child {
            Result::Ok(_) => {
                let op = child?.wait_with_output().await?;
                let op_utf = str::from_utf8(&op.stdout).unwrap();
                if op_utf.contains(ADB_VERSION_RESULT) {
                    Ok(true)
                }
                else {
                    Err(anyhow!("Something Strange!"))
                }
            }
            Err(_e) => {
                Ok(false)
            }
        }
    }
}