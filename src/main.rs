mod args;

fn main() {
    let _ = args::parse_args();

    // veryfynig if  has the api key for main process
    let api_key = args::verify_has_api_key().map_err(|err |{
            if err.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Error: config file not found\n Please get your api key here https://www.weatherapi.com/ \n and run `wfetch --api-key <api_key>`");
                std::process::exit(1);
            }
    } ).unwrap();
    println!("{:#?}", api_key);
    println!("idk, but why?");
}
