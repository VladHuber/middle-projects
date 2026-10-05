use crate::models::Todo;
use rusqlite::{Connection, Result, params};

pub fn insert(conn: &Connection, todo: &Todo) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks(title, is_done) VALUES (?,?)",
        params![todo.title, todo.done as i32],
    )?;
    Ok(())
}

pub fn select(conn: &Connection) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare("SELECT id, title, is_done FROM tasks ORDER BY id")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            done: row.get::<_, i32>(2)? != 0,
        })
    })?;
    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

pub fn update_mark(conn: &Connection, index: i64) -> Result<()> {
    conn.execute("UPDATE tasks SET is_done=1 WHERE id=?", [index])?;
    Ok(())
}

pub fn delete_elm(conn: &Connection, index: i64) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id=?", [index])?;
    Ok(())
}
