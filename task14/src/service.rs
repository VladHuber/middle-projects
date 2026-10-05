use crate::error::AppResult;
use crate::parser;

pub fn show_config() -> AppResult<()>{
    let toml_doc = parser::deserialize()?;    
    println!("{:#?}", toml_doc);
    Ok(())
}

pub fn set_data(key:&str, val:&str) -> AppResult<()> {
    let mut toml_doc = parser::deserialize()?;    
    toml_doc.set_value(key, val)?;
    parser::serialize(&toml_doc)?;
    println!("Данные успешно изменены");
    Ok(())
}