use std::{env, error::Error};

mod commands;
mod manifest;

use crate::commands::{init, install, read, remove, search, update};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1);
    match command {
        Some(command) if command == "init" => {
            init()?;
        }
        Some(command) if command == "read" => {
            read()?;
        }
        Some(command) if command == "install" => {
            let package = args.get(2).expect("Нету пакета установочного");
            install(package)?;
        }
        Some(command) if command == "remove" => {
            let package = args.get(2).expect("Нету пакета установочного");
            remove(package)?;
        }
        Some(command) if command == "search" => {
            let package = args.get(2).expect("Нету пакета установочного");
            search(package)?;
        }
        Some(command) if command == "update" => {
            update()?;
        }
        _ => {}
    }
    Ok(())
}
