use std::fs;
use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package{
    pub name: String,
    pub version: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Cargo{
    pub package: Package,
    pub dependencies: BTreeMap<String,Dependency> 
}
#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)]
pub enum Dependency  {
    Simple(String),
    Detailed{
        version: String,
        features: Option<Vec<String>>
    }
}
impl Dependency {
    pub fn version(&self) -> &str {
        match self {
            Dependency::Detailed { version, .. } => version,
            Dependency::Simple(version) => version,
        }
    }
}
pub fn create_cargo(path: String) -> Result<(), Box<dyn std::error::Error>>{
    let cargo_file = Cargo{
        package: Package { name: "my_project".to_string(), version: "0.1.0".to_string() },
        dependencies: BTreeMap::new()
    };
    let toml_text = toml::to_string_pretty(&cargo_file)?;
    fs::write(path, toml_text)?;
    Ok(())
}

pub fn read_cargo() -> Result<Cargo, Box<dyn std::error::Error>>{
    let path = "my_project/Cargo.toml".to_string();
    let cargo_file = fs::read_to_string(path)?;
    let cargo: Cargo = toml::from_str(&cargo_file)?;
    Ok(cargo)
}