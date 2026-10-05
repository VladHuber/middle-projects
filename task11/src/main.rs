mod db;
mod models;
mod repository;
mod service;

use crate::models::Todo;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if require_arg(&args, 1).is_none() {
        eprintln!("Мало аргументов");
        return Ok(());
    }
    let conn = db::db_connect()?;
    match args.get(1).map(String::as_str) {
        Some("add") => {
            if require_arg(&args, 2).is_none() {
                eprintln!("Мало аргументов");
                return Ok(());
            }
            service::add_todo(&conn, &args[2])?
        }
        Some("list") => {
            let tasks = service::list(&conn)?;
            print_tasks(&tasks);
        }
        Some("done") => {
            if require_arg(&args, 2).is_none() {
                eprintln!("Мало аргументов");
                return Ok(());
            }
            let index: i64 = args[2].parse()?;
            service::mark_done(&conn, index)?
        }
        Some("delete") => {
            if require_arg(&args, 2).is_none() {
                eprintln!("Мало аргументов");
                return Ok(());
            }
            let index: i64 = args[2].parse()?;
            service::delete_todo(&conn, index)?
        }
        _ => {
            eprintln!("Неизвестная команда");
        }
    }
    Ok(())
}

fn require_arg(args: &[String], index: usize) -> Option<&str> {
    args.get(index).map(|s| s.as_str())
}

fn print_tasks(tasks: &[Todo]) {
    println!("\n--- Список задач ---");
    for task in tasks {
        println!("{} {} {}", task.id, task.title, task.done);
    }
}
