use chrono::{Local, TimeZone};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, fs, process};
use toml;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    name: String,
    main: MainInfo,
    sys: SysInfo,
    coord: CoordInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct MainInfo {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
    feels_like: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct SysInfo {
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct CoordInfo {
    lon: f64,
    lat: f64,
}

#[derive(Debug)]
struct WeatherData {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
    temp_feels_like: f64,
    city_name: String,
    country_name: String,
    sunrise: String,
    sunset: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Answers {
    country: String,
    city: String,
    unit_temp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    openweather_api_key: String,
}

#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    handle_args();
    let answers: Answers = get_info();
    let data = fetch_api(&answers).await.unwrap();
    format_data(data, answers.unit_temp);
    Ok(())
}

fn handle_args() -> () {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(first_arg) => match first_arg.as_str() {
            "--clear" | "-cl" => clear_config(),
            "--help" | "-h" => {
                println!(
                    "fetch-rs help\n\n\
          Usage: wfetch --command\n\n\
          \x1b[32m--help\x1b[0m\n\
          display this message.\n\n\
          \x1b[32m--clear\x1b[0m\n\
          delete the config file."
                );
                process::exit(0);
            }
            _ => {
                println!("\x1b[31mUnknown Option: {}", first_arg);
                process::exit(1);
            }
        },
        None => (),
    }
}

fn clear_config() -> () {
    let home_dir = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("Could not determine home directory"));
    let file_path = home_dir.join(".config/wfetchrs.json");
    match fs::remove_file(file_path) {
        Ok(()) => {
            println!("\x1b[32mConfig file deleted");
            process::exit(0);
        }
        Err(e) => {
            println!("\x1b[31mError: {}", e);
            process::exit(1);
        }
    }
}

fn get_info() -> Answers {
    let answers: Answers;

    let home_dir = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("Could not determine home directory"));
    let file_path = home_dir.join(".config/wfetchrs.json");

    if fs::metadata(&file_path).is_ok() {
        let contents = fs::read_to_string(file_path).expect("Error reading file");
        answers = serde_json::from_str(&contents).expect("Error parsing JSON");
    } else {
        answers = prompt_questions();
        let serialized = serde_json::to_string(&answers).expect("Error serializing JSON");
        fs::write(file_path, serialized).expect("Error writing file");
    }
    return answers;
}

fn prompt_questions() -> Answers {
    let mut city = String::new();
    let mut country = String::new();
    let mut unit_temp = String::new();
    let unit_temps: Vec<&str> = vec![" Celsius ", " Fahrenheit ", " Kelvin "];

    loop {
        city.clear();
        country.clear();
        unit_temp.clear();

        print!("City: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut city).unwrap();

        print!("Country: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut country).unwrap();

        for (index, unit) in unit_temps.iter().enumerate() {
            print!("{} >", index + 1);
            println!("{:}", unit);
        }
        print!("Unit Temp: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut unit_temp).unwrap();

        print!("Correct Data? (y/n): ");
        io::stdout().flush().unwrap();
        let mut retry = String::new();
        io::stdin().read_line(&mut retry).unwrap();

        match retry.trim().to_lowercase().as_str() {
            "sim" | "s" | "yes" | "y" | "verdadeiro" | "v" => {
                print!("\x1B[2J\x1B[1;1H");
                break;
            }
            "não" | "n" | "no" | "false" | "falso" | "f" => continue,
            _ => panic!("a"),
        };
    }
    let answers = Answers {
        city: city.trim().to_string(),
        country: country.trim().to_string(),
        unit_temp: unit_temp.trim().to_string(),
    };
    return answers;
}

async fn fetch_api(answers: &Answers) -> Result<WeatherData, Box<dyn std::error::Error>> {
    const CONFIG_TOML: &'static str = include_str!("../config.toml");
    let config: ConfigFile = toml::from_str(CONFIG_TOML).unwrap();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&APPID={}",
        answers.city, answers.country, config.openweather_api_key
    );
    let data = reqwest::get(&url).await?.json::<Data>().await?;
    Ok(WeatherData {
        temp: data.main.temp,
        temp_min: data.main.temp_min,
        temp_max: data.main.temp_max,
        temp_feels_like: data.main.feels_like,
        city_name: data.name,
        country_name: data.sys.country,
        sunrise: Local
            .timestamp(data.sys.sunrise, 0)
            .format("%H:%M:%S")
            .to_string(),
        sunset: Local
            .timestamp(data.sys.sunset, 0)
            .format("%H:%M:%S")
            .to_string(),
    })
}

fn format_data(data: WeatherData, unit_temp: String) -> () {
    println!("Country: {}", data.country_name);
    println!("City: {}", data.city_name);

    if unit_temp == "1" {
        println!("Temp: {:.1} °C", (data.temp - 273.0));
        println!("Temp Max: {:.1} °C", (data.temp_max - 273.0));
        println!("Temp Min: {:.1} °C", (data.temp_min - 273.0));
        println!("Feels Like: {:.1} °C", (data.temp_feels_like - 273.0));
    } else if unit_temp == "2" {
        println!("Temp: {:.1} °F", ((data.temp - 273.15) * 1.8 + 32.0));
        println!(
            "Temp Max: {:.1} °F",
            ((data.temp_max - 273.15) * 1.8 + 32.0)
        );
        println!(
            "Temp Min: {:.1} °F",
            ((data.temp_min - 273.15) * 1.8 + 32.0)
        );
        println!(
            "Feels Like: {:.1} °F",
            ((data.temp_feels_like - 273.15) * 1.8 + 32.0)
        );
    } else if unit_temp == "3" {
        println!("Temp: {:.1} °C", data.temp);
        println!("Temp Max: {:.1} °C", data.temp_max);
        println!("Temp Min: {:.1} °C", data.temp_min);
        println!("Feels Like: {:.1} °C", data.temp_feels_like);
    }

    println!("Sunrise: {}", data.sunrise);
    println!("Sunset: {}", data.sunset);
}
