use std::{fs, path::PathBuf, error::Error, fmt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use serde::de::Error;
use csv;

// Leaving as a reference for more fields if needed.
// typeID,groupID,typeName,description,mass,volume,capacity,portionSize,raceID,basePrice,published,marketGroupID,iconID,soundID,graphicID
// pub struct InvType {
//     type_id: usize,
//     group_id: usize,
//     type_name: String,
//     desc: String,
//     mass: f64,
//     volume: f64,
//     capacity: f64,
//     portion_size: usize,
//     race_id: usize,
//     base_price: usize,
//     published: usize,
//     market_group_id: usize,
//     icon_id: usize,
//     sound_id: usize,
//     graphic_id: usize,
// }

#[derive(Debug, Deserialize)]
pub struct InvType {
    #[serde(rename = "typeID")]
    pub type_id: usize,
    #[serde(rename = "groupID")]
    pub group_id: usize,
    #[serde(rename = "typeName")]
    pub type_name: String,
}

pub fn read_inv_types(inv_types_path: PathBuf) -> Result<HashMap<usize, InvType>, Box<dyn Error>> {
    let mut types_by_id: HashMap<usize, InvType> = Default::default();
    let mut rdr = csv::Reader::from_path(&inv_types_path)?;
    for result in rdr.deserialize() {
        let record: InvType = result?;
        types_by_id.insert(record.type_id, record);
    }
    Ok(types_by_id)
}

pub fn get_name_map(types_by_id: HashMap<usize, InvType>) -> HashMap<String, InvType>{
    let mut types_by_name: HashMap<String, InvType> = Default::default();

    for (_key, value) in types_by_id.into_iter() {
        let name: String = value.type_name.clone();
        types_by_name.insert(name, value);
    } 

    return types_by_name;
}