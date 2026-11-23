use rusqlite::Connection;

use crate::error::AppResult;

// Append-only: never edit or reorder an entry once released. The number of
// applied entries is stored in SQLite's `user_version` pragma.
const MIGRATIONS: &[&str] = &[
    include_str!("migrations/0001_create_items.sql"),
    include_str!("migrations/0002_add_item_details.sql"),
];

pub fn run(conn: &mut Connection) -> AppResult<()> {
    let applied: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    let applied = applied as usize;

    for (index, sql) in MIGRATIONS.iter().enumerate().skip(applied) {
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        // PRAGMA statements cannot take bound parameters.
        tx.execute_batch(&format!("PRAGMA user_version = {};", index + 1))?;
        tx.commit()?;
    }
    Ok(())
}
