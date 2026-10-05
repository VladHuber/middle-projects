mod error;
mod service;

use std::{
    env::{self},
    path::Path,
};
// cargo run -- archive create zipf2.zip .\folder
// cargo run -- archive list zipf.zip
// cargo run -- archive extract zipf.zip .\folder

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Мало аргументов");
        return Ok(());
    }
    let command = &args[1];
    if command == "archive" {
        let mode = &args[2];
        match mode.as_str() {
            "create" => {
                if args.len() < 5 {
                    eprintln!("Мало аргументов");
                    return Ok(());
                }
                let zip_name = &args[3];
                let folder_path = &args[4];
                let target_path = match Path::new(folder_path).canonicalize() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Ошибка: Папка '{}' не найдена ({})", folder_path, e);
                        return Ok(());
                    }
                };
                let target_dir_str = target_path.to_string_lossy();
                service::create(zip_name, &target_dir_str)?;
            }
            "list" => {
                if args.len() < 3 {
                    eprintln!("Мало аргументов");
                    return Ok(());
                }
                let zip_file = &args[3];    
                service::list(zip_file)?;
            }
            "extract" => {
                if args.len() < 5 {
                    eprintln!("Мало аргументов");
                    return Ok(());
                }
                let zip_name = &args[3];
                let folder_path = &args[4];
                service::extract(zip_name, folder_path)?;
            }
            _ => {}
        }
    } else {
        println!("Неизвестная команда попробуйте вписать: cargo run -- archive");
    }

    Ok(())
}
