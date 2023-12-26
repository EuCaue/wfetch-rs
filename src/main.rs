mod args;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    tz_id: String,
    localtime_epoch: i32,
    localtime: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Current {
    last_updated_epoch: i32,
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    is_day: i32,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: i32,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: i32,
    cloud: i32,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseLocation {
    location: Location,
    current: Current,
}

#[derive(Debug)]
struct LocalTime {
    twelve_hour: String,
    twenty_four_hour: String,
}

#[derive(Debug)]
struct FormatedData {
    location_name: String,
    weather_c: f32,
    weather_f: f32,
    feelslike_c: f32,
    feelslike_f: f32,
    localtime: LocalTime,
    wind_mph: f32,
    wind_kph: f32,
    humidity: i32,
    precip_mm: f32,
}

fn convert_to_12h_format(time_24h: String) -> String {
    let parts: Vec<&str> = time_24h.split(':').collect();

    if parts.len() == 2 {
        let mut hour = parts[0].parse::<u32>().unwrap_or(0);
        let minute = parts[1];

        let period = if hour < 12 { "AM" } else { "PM" };

        if hour == 0 {
            hour = 12;
        } else if hour > 12 {
            hour -= 12;
        }

        format!("{:02}:{:} {}", hour, minute, period)
    } else {
        "Invalid time".to_string()
    }
}

fn get_formated_data(response_location: ResponseLocation) -> FormatedData {
    let location_name = format!(
        "{city},{state},{country}",
        city = response_location.location.name,
        state = response_location.location.region,
        country = response_location.location.country
    );
    let twelve_hour = convert_to_12h_format(response_location.location.localtime.clone());
    let twenty_four_hour = response_location
        .location
        .localtime
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .to_string();

    let fetch_data = FormatedData {
        location_name,
        weather_c: response_location.current.temp_c,
        weather_f: response_location.current.temp_f,
        feelslike_c: response_location.current.feelslike_c,
        feelslike_f: response_location.current.feelslike_f,
        localtime: LocalTime {
            twelve_hour,
            twenty_four_hour,
        },
        wind_mph: response_location.current.wind_mph,
        wind_kph: response_location.current.wind_kph,
        humidity: response_location.current.humidity,
        precip_mm: response_location.current.precip_mm,
    };
    fetch_data
}

fn get_location_data(
    api_key: String,
    location: String,
) -> Result<ResponseLocation, Box<dyn std::error::Error>> {
    let formated_url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        api_key, location
    );
    let response_location = reqwest::blocking::get(formated_url)?.json::<ResponseLocation>()?;
    println!("{:#?}", response_location);
    Ok(response_location)
}

fn display_formated_data(data: FormatedData) -> () {
    println!("  Location:          {}", data.location_name);
    println!(
        "  Weather:           {:.1} °C || {:.1} °F",
        data.weather_c, data.weather_f
    );
    println!(
        "  Feels like:        {:.1} °C || {:.1} °F",
        data.feelslike_c, data.feelslike_f
    );
    println!(
        "  Local time:        {} || {}",
        data.localtime.twenty_four_hour, data.localtime.twelve_hour
    );
    println!(
        "  Wind:              {} kph || {} mph",
        data.wind_kph, data.wind_mph
    );
    println!("  Humidity:          {} %", data.humidity);
    println!("  Precipitation:     {} mm", data.precip_mm);
}

fn main() {
    let _ = args::parse_args();
    // veryfynig if  has the api key for main process
    let api_key = args::verify_has_api_key().map_err(|err |{
        // error when the file config file it's not found 
            if err.kind() == std::io::ErrorKind::NotFound {

            println!("Error: {}", err);
                eprintln!("Error: config file not found\n Please get your api key here https://www.weatherapi.com/ \n and run `wfetch --api-key <api_key>`");
                std::process::exit(1);
            }
    } ).unwrap();
    let (config, _) = args::read_config_file().unwrap();
    let location_data = get_location_data(api_key, config.QUERY_LOCATION.unwrap()).unwrap();
    let data = get_formated_data(location_data);
    display_formated_data(data)
}
