use anyhow::anyhow;
use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::env::current_dir;
use quest2deployer::{adb::{self, Manager}, io, zip};

#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    adb_device: String,
    apk_path: String
}

fn init_config() -> Result<Option<Config>, anyhow::Error> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    
    let adb_device = Input::with_theme(&theme)
        .with_prompt("Interface")
        .default("127.0.0.1".parse().unwrap())
        .interact()?;

    let apk_path = Input::with_theme(&theme)
        .with_prompt("Interface")
        .default("127.0.0.1".parse().unwrap())
        .interact()?;

    println!("Package Name: {}", adb_device);

    Ok(Some(Config {
        adb_device,
        apk_path
    }))
}

async fn setup_adbtools(adb_manager : &mut Manager) -> Result<(), anyhow::Error> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let adb_result = adb_manager.check_for_adb().await;
    match adb_result {
        Ok(_) => {
         return Ok(())  
        }
        Err(_e) => {
            println!(
                "ADB TOOLS NOT FOUND!: {:010x}",
                style(42).red().on_black().bold()
            );
            let confirmation = Confirm::with_theme(&theme)
            .with_prompt("Download ADB Tools Now?")
            .interact()
            .unwrap();
        
            if confirmation {
                println!("Looks like you want to continue");
            } else {
                return Err(anyhow!("Escape!"));
            };
            
            let client = reqwest::Client::new();
        
            let mut out_dir = current_dir()
                .expect("Failed to get current working directory");
        
            let downloaded_zip = io::download_url(&client,"https://dl.google.com/android/repository/platform-tools-latest-darwin.zip", "adbtools.zip").await.unwrap();
            zip::unzip_file(downloaded_zip, &out_dir).await;
            out_dir.extend(&["platform-tools", "adb"]);
        
            Ok(())
        }
    }
    
}

#[tokio::main]
async fn main() {
    let mut adb_manager = adb::Manager::new();
    match setup_adbtools(&mut adb_manager).await {
        Ok(_) => println!("Yeha"),
        Err(_err) => return,
    }
    println!("{:?}", adb_manager);
    match init_config() {
        Ok(None) => println!("Aborted."),
        Ok(Some(config)) => println!("{:#?}", config),
        Err(err) => println!("error: {}", err),
    }
}