use std::{fs, path::PathBuf, error::Error, fmt};

mod config;
mod inv_types;

fn main() {
    let config_path = PathBuf::from(r"C:\Users\wario\Documents\GitHub\SupplySergeantV2\config.toml");
    let config:config::Config = config::read_config(config_path);
    // println!("{:?}", config);
    let types_by_id: std::collections::HashMap<usize, inv_types::InvType> = inv_types::read_inv_types(config.global_settings.inv_types_path).unwrap();
    // println!("{:?}", &types_by_id);
    let types_by_name = inv_types::get_name_map(types_by_id);
    println!("{:?}", &types_by_name);
}
