mod commands;
mod objects;
mod index;

use std::env;
use crate::commands::{init, add, status, commit, log};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    let command = args.get(1);
    match command {
        Some(command) if command == "init" => {
            init()?;
        }
        Some(command) if command == "add" => {
            let file = args.get(2).expect("Укажите файл");
            add(file)?;
        }

        Some(command) if command == "status" => {
            status()?;
        }

        Some(command) if command == "commit" => {
            let message = args.get(2).expect("Укажите комментарий");
            commit(message)?;
        }

        Some(command) if command == "log" => {
            log()?;
        }
        _ => {
            eprintln!("Неизвестная команда");
        }
    }

    Ok(())
}
