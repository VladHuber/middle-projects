use std::fs;

use crate::models::Config;
use crate::error::AppResult;

const DIR_CONFIG_FILE : &str = "./configs/config.toml"; 

pub fn deserialize() -> AppResult<Config>{
    let file = fs::read_to_string(DIR_CONFIG_FILE)?;
    let toml_struct : Config = toml::from_str(&file)?;
    Ok(toml_struct)
}

pub fn serialize(config: &Config) -> AppResult<()>{
    let toml_doc = toml::to_string_pretty(&config)?;
    fs::write(DIR_CONFIG_FILE, toml_doc)?;
    Ok(())
}