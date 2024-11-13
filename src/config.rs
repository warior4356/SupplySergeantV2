use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml;
use crate::config::toml::de::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemSettings {
    inv_types_path: PathBuf,
    corporation_id: usize,
    fits_path: PathBuf,
    gsheets_client_secret_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ESISettings {
    agent_id: String,
    auth_code: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetSystem {
    check_contracts: bool,
    check_market: bool,
    citadel_ids: Vec<usize>,
    gsheets_starting_page: usize,
    market_reference_region: usize,
    output_file_name: String,
    region_id: usize,
    ships: HashMap<String, usize>,
    items: HashMap<String, usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    system_settings: SystemSettings,
    esi_settings: ESISettings,
    target_systems: HashMap<String, TargetSystem>,
}

pub fn read_config(config_path: PathBuf) -> Config {
    let toml_string = match fs::read_to_string(&config_path) { 
        Ok(c) => c,

        Err(_) => {
            eprintln!("Could not read file `{:?}`", config_path.to_str());
            eprintln!("Could not read file");
            std::process::exit(1);
        }
    };
    
    // println!("{:?}", toml_string);
    // let config: HashMap<String, String> = toml::from_str(toml_string.as_str()).unwrap();
    // let config: Config = toml::from_str(toml_string.as_str()).unwrap();
    // println!("{:?}", config);
    let config: Config = match toml::from_str::<Result<Config, Error>>(&toml_string.as_str()).unwrap(){ // This line wouldn't work without result defined, but if I define it like this, it says error does not satisfy the trait deserialize
        Ok(c) => c,

        Err(_) => {
            eprintln!("Could not parse toml string");
            std::process::exit(1);
        }
    };
    return config;
}