use core::str;
use std::process::Stdio;

use anyhow::anyhow;
use tokio::process::Command;

pub async fn check_for_adb() -> Result<(), anyhow::Error> {

    const ADB_VERSION_RESULT: &str = "Android Debug Bridge version";

    let mut cmd = Command::new("adbb");

    cmd.arg("version");

    // Specify that we want the command's standard output piped back to us.
    // By default, standard input/output/error will be inherited from the
    // current process (for example, this means that standard input will
    // come from the keyboard and standard output/error will go directly to
    // the terminal if this process is invoked from the command line).
    
    cmd.stdout(Stdio::piped());

    let child = cmd.spawn();
    match child {
        Ok(_) => {
            let op = child?.wait_with_output().await?;
            let op_utf = str::from_utf8(&op.stdout).unwrap();
            println!("{}", op_utf);
            if op_utf.contains(ADB_VERSION_RESULT) {
                Ok(())
            }
            else {
                Err(anyhow!("Something Strange!"))
            }
        }
        Err(_e) => {
            Err(anyhow!("ADB Tools Not Found!"))
        }
    }


    
    
    
}

