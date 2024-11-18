use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml;

// Base program settings struct
#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalSettings {
    pub inv_types_path: PathBuf,
    pub corporation_id: usize,
    pub fits_path: PathBuf,
    pub gsheets_client_secret_path: PathBuf,
}

// Settings for ESI connection
#[derive(Debug, Deserialize, Serialize)]
pub struct ESISettings {
    pub agent_id: String,
    pub auth_code: String,
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

// Target system specific setting
#[derive(Debug, Deserialize, Serialize)]
pub struct TargetSystem {
    pub check_contracts: bool,
    pub check_market: bool,
    pub citadel_ids: Vec<usize>,
    pub gsheets_starting_page: usize,
    pub market_reference_region: usize,
    pub output_file_name: String,
    pub region_id: usize,
    pub ships: HashMap<String, usize>,
    pub items: HashMap<String, usize>,
}

// Struct for all settings combined
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub global_settings: GlobalSettings,
    pub esi_settings: ESISettings,
    pub target_systems: HashMap<String, TargetSystem>,
}

// Read config file into struct and return it
pub fn read_config(config_path: PathBuf) -> Config { // Make return a result instead of exiting???
    let toml_string = match fs::read_to_string(&config_path) { 
        Ok(c) => c,

        Err(_) => {
            eprintln!("Could not read file `{:?}`", config_path.to_str());
            std::process::exit(1);
        }
    };
    
    let config: Config = match toml::from_str(&toml_string) {
        Ok(c) => c,

        Err(_) => {
            eprintln!("Could not parse toml string from `{:?}`", config_path.to_str());
            std::process::exit(1);
        }
    };
    return config;
}