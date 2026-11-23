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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::item::{ItemStatus, Priority};
    use crate::repositories::item_repo::ItemRepository;

    #[test]
    fn upgrades_rows_created_by_the_first_migration() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(MIGRATIONS[0]).unwrap();
        conn.execute_batch("PRAGMA user_version = 1;").unwrap();
        conn.execute(
            "INSERT INTO items (id, title, status, created_at, updated_at)
             VALUES ('old', 'from step 0', 'done', 1, 2)",
            [],
        )
        .unwrap();

        run(&mut conn).unwrap();

        let item = ItemRepository::find(&conn, "old").unwrap().unwrap();
        assert_eq!(item.title, "from step 0");
        assert_eq!(item.status, ItemStatus::Done);
        assert_eq!(item.due_date, None);
        assert_eq!(item.due_time, None);
        assert_eq!(item.note, "");
        assert_eq!(item.priority, Priority::Normal);
        assert_eq!(item.updated_at, 2);
    }

    #[test]
    fn running_twice_is_a_no_op() {
        let mut conn = Connection::open_in_memory().unwrap();
        run(&mut conn).unwrap();
        run(&mut conn).unwrap();
        let version: i64 = conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version as usize, MIGRATIONS.len());
    }
}
