use rusqlite::Connection;

use crate::models::Todo;
use crate::repository;

use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn add_todo(conn: &Connection, task: &str) -> AppResult<()> {
    let todo = Todo {
        id: 0,
        title: task.to_string(),
        done: false,
    };
    repository::insert(conn, &todo)?;
    Ok(())
}

pub fn list(conn: &Connection) -> AppResult<Vec<Todo>> {
    let tasks = repository::select(conn)?;
    Ok(tasks)
}

pub fn mark_done(conn: &Connection, index: i64) -> AppResult<()> {
    repository::update_mark(conn, index)?;
    Ok(())
}
pub fn delete_todo(conn: &Connection, index: i64) -> AppResult<()> {
    repository::delete_elm(conn, index)?;
    Ok(())
}
