use std::env;

use crate::error::AppResult;

mod error;
mod models;
mod parser;
mod service;
mod validator;

fn main() -> AppResult<()> {
    let args: Vec<String> = env::args().collect();
    
    if require_arg(&args, 1).is_none() {
        eprintln!("Мало аргументов");
        return Ok(());
    }
    let command = &args[1];

    match command.as_str() {
        "show" => {
            service::show_config()?;
        }
        "set" => {
            if require_arg(&args, 2).is_none() || require_arg(&args, 3).is_none() {
                eprintln!("Не заполнены данные key value");
                return Ok(());
            }
            let key = &args[2];
            let value = &args[3];
            if let Err(err_message) = service::set_data(key, value){
                eprintln!("{err_message}");
                std::process::exit(1);
            }

        }
        _ => {eprintln!("Неизвестная команда");}
    }

    Ok(())
}

fn require_arg(args: &[String], index: usize) -> Option<&str> {
    args.get(index).map(|s| s.as_str())
}
