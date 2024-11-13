use std::path::PathBuf;

mod config;

fn main() {
    let config_path = PathBuf::from(r"C:\Users\wario\Documents\GitHub\SupplySergeantV2\config.toml");
    let config:config::Config = config::read_config(config_path);
    println!("{:?}", config);
}
