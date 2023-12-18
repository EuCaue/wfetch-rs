use clap::{arg, command};
use home::home_dir;
use requestty::Question;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};

#[derive(Debug)]
pub struct Args {
    pub api_key: Option<String>,
    pub setup: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    API_KEY: String,
    QUERY_LOCATION: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Location {
    id: u64,
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    url: String,
}

fn get_config_path() -> std::path::PathBuf {
    let config_path = ".config";
    let config_file = "wfetch.json";
    let home_dir = home_dir().unwrap();
    let path = home_dir.join(config_path).join(config_file);
    return path;
}

fn read_config_file() -> std::io::Result<(ConfigFile, File)> {
    let path = get_config_path();
    let mut config_file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    let config: ConfigFile = serde_json::from_str(&config_content)?;
    Ok((config, config_file))
}

fn update_field_in_json(field: &str, new_value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (mut config, mut config_file) = read_config_file()?;
    // println!("{:#?}", config);
    // println!("{:#?}", config_file);

    match field.to_lowercase().as_str() {
        "api_key" => config.API_KEY = new_value.to_string(),
        "query_location" => config.QUERY_LOCATION = Some(new_value.to_string()),
        _ => {
            eprintln!("Unknown field: {}", field);
            return Ok(());
        }
    }

    let updated_config_string = serde_json::to_string(&config)?;
    config_file.seek(SeekFrom::Start(0))?;
    config_file.write_all(updated_config_string.as_bytes())?;
    config_file.set_len(updated_config_string.len() as u64)?;
    Ok(())
}

pub fn verify_has_api_key() -> std::io::Result<String> {
    let (config, _) = read_config_file()?;
    Ok(config.API_KEY)
}

fn get_setup_location(api_key: String) -> Result<Location, Box<dyn std::error::Error>> {
    let question_location = Question::input("Location")
        .message("Search your location")
        .build();

    let location = requestty::prompt_one(question_location).unwrap();

    let formated_url = format!(
        "http://api.weatherapi.com/v1/search.json?key={}&q={}",
        api_key,
        location.as_string().unwrap()
    );
    let response_locations = reqwest::blocking::get(formated_url)?.json::<Vec<Location>>()?;
    let locations_formated: Vec<String> = response_locations
        .iter()
        .map(|location| {
            format!(
                "{}, {}, {}",
                location.name, location.region, location.country
            )
        })
        .collect();

    let question_select = Question::select("City")
        .message("Choose the location?")
        .choices::<Vec<String>, _>(locations_formated)
        .build();

    let answer_location = requestty::prompt_one(question_select).unwrap();
    let index = answer_location.as_list_item().unwrap().index;
    let chosen_location = response_locations.clone().get(index).unwrap().clone();
    println!("Response: {:#?}", answer_location);
    Ok(chosen_location)
}

fn handle_setup() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = verify_has_api_key()?;
    let location = get_setup_location(api_key)?;
    let update_field = "QUERY_LOCATION";

    match update_field_in_json(update_field, location.url.as_str()) {
        Err(err) => {
            println!("Error: {}", err);
            eprintln!("Error: config file not found\n Please get your api key here https://www.weatherapi.com/ \n and run `wfetch --api-key <api_key>`");
            std::process::exit(1);
        }
        Ok(_) => (),
    }
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
        match update_field_in_json("api_key", api_key) {
            Ok(_) => {}
            Err(_) => {
                let path = get_config_path();
                let mut config_file = File::create(path.clone())?;
                let my_data = json!(
                {
                  "API_KEY": api_key,
                }
                );
                let json_string = serde_json::to_string(&my_data)?;
                config_file.write_all(json_string.as_bytes())?;
            }
        }
    }

    if let Some(setup) = matches.get_one::<bool>("setup") {
        if *setup {
            let _ = handle_setup().map_err(|_err| {
                // TODO: do a better error matching here
                println!("Error: {}", _err);
                eprintln!("Error: config file not found\n Please get your api key here https://www.weatherapi.com/ \n and run `wfetch --api-key <api_key>` from here?");
                std::process::exit(1);
            });
        }
    }

    Ok(Args {
        api_key: matches.get_one::<String>("api_key").cloned(),
        setup: matches.get_one::<bool>("setup").map(|s| s.to_string()),
    })
}
