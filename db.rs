use rusqlite;
use rusqlite::{params, Connection};

fn test_sqlite() -> Result<(), Box<dyn Error>> {
    // test sqlite
    println!("sqlite: {}", rusqlite::version());
    let conn = Connection::open("./matter.db")?;
    let mut stmt =
        conn.prepare("INSERT INTO posts (user_id, channel_id, message) values (?, ?, ?)")?;
    stmt.execute(&["u123", "c456", "plop 1"])?;
    stmt.execute(&["u123", "c456", "plop 2"])?;
    stmt.execute(&["u123", "c456", "plop 3"])?;
    stmt.execute(&["u123", "c456", "plop 4"])?;
    return Ok(());
}
