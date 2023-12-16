use clap::{arg, command};
use home::home_dir;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Args {
    pub api_key: Option<String>,
    pub setup: Option<String>,
}

pub fn verify_has_api_key() -> std::io::Result<String> {
    let home_dir = home_dir().unwrap();
    let path = home_dir.join(".config").join("wfetch.cfg");
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//  TODO: do question here
fn handle_setup() -> std::io::Result<()> {
    let api_key = verify_has_api_key()?;
    // let formated_url = format!(
    //     "http://api.weatherapi.com/v1/search.json?key={}&q=salvador",
    //     api_key
    // );
    // let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    Ok(())
}

pub fn parse_args() -> std::io::Result<Args> {
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
             -k --api_key <API_KEY> "Sets the api key for the api"
            )
            .required(false),
        )
        .arg(
            arg!(
                -s --setup "Setup the application"
            )
            .required(false),
        )
        .get_matches();

    //  TODO: some way to verify the api key
    if let Some(api_key) = matches.get_one::<String>("api_key") {
        let home_dir = home_dir().unwrap();
        let path = home_dir.join(".config").join("wfetch.cfg");
        let mut config_file = File::create(path.clone())?;
        config_file.write_all(api_key.as_bytes())?;
        println!("api key: {api_key}");
        println!("{home}", home = path.display());
    }

    if let Some(setup) = matches.get_one::<bool>("setup") {
        if *setup {
            let _ = handle_setup().map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Error: config file not found\n Please get your api key here https://www.weatherapi.com/ \n and run `wfetch --api-key <api_key>`");
                std::process::exit(1);
            }
        });
        }
    }

    Ok(Args {
        api_key: matches.get_one::<String>("api_key").cloned(),
        setup: matches.get_one::<bool>("setup").map(|s| s.to_string()),
    })
}
