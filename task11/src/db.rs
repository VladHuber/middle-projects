use rusqlite::{Connection, Result};
const DB_NAME: &str = "mydb.db";

pub fn db_connect() -> Result<Connection> {
    let conn = Connection::open(DB_NAME)?;
    check_tables(&conn)?;

    Ok(conn)
}
fn check_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            is_done INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}
