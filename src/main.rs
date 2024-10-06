use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};


mod io;

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

async fn setup_adbtools() -> Result<bool, anyhow::Error> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    
    let confirmation = Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap();

    if confirmation {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
        return Ok(false);
    };
    let client = reqwest::Client::new();
    io::download_url(&client,"https://dl.google.com/android/repository/platform-tools-latest-darwin.zip", "adbtools.zip").await.unwrap();


    Ok(true)
}

#[tokio::main]
async fn main() {
    match setup_adbtools().await {
        Ok(true) => println!("continue!"),
        Ok(false) => {
            println!("Aborted.");
            return;
        },
        Err(err) => println!("error: {}", err),
    }
    match init_config() {
        Ok(None) => println!("Aborted."),
        Ok(Some(config)) => println!("{:#?}", config),
        Err(err) => println!("error: {}", err),
    }
}