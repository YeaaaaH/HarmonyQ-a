use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::error::AppResult;
use crate::models::item::{Item, ItemStatus, Priority};

const COLUMNS: &str =
    "id, title, status, due_date, due_time, note, priority, created_at, updated_at";

pub struct ItemRepository;

impl ItemRepository {
    pub fn list(conn: &Connection) -> AppResult<Vec<Item>> {
        let mut stmt = conn.prepare(&format!(
            "SELECT {COLUMNS} FROM items ORDER BY created_at DESC"
        ))?;
        let items = stmt
            .query_map([], map_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    /// Open tasks due on or before `date`, plus every task due exactly on `date`.
    pub fn list_due_by(conn: &Connection, date: &str) -> AppResult<Vec<Item>> {
        let mut stmt = conn.prepare(&format!(
            "SELECT {COLUMNS} FROM items
             WHERE due_date = ?1 OR (status = 'open' AND due_date < ?1)"
        ))?;
        let items = stmt
            .query_map(params![date], map_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    pub fn find(conn: &Connection, id: &str) -> AppResult<Option<Item>> {
        let item = conn
            .query_row(
                &format!("SELECT {COLUMNS} FROM items WHERE id = ?1"),
                params![id],
                map_row,
            )
            .optional()?;
        Ok(item)
    }

    pub fn insert(conn: &Connection, item: &Item) -> AppResult<()> {
        conn.execute(
            &format!("INSERT INTO items ({COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"),
            params![
                item.id,
                item.title,
                item.status.as_str(),
                item.due_date,
                item.due_time,
                item.note,
                item.priority.as_str(),
                item.created_at,
                item.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn update_details(conn: &Connection, item: &Item) -> AppResult<usize> {
        let changed = conn.execute(
            "UPDATE items
             SET title = ?1, due_date = ?2, due_time = ?3, note = ?4, priority = ?5, updated_at = ?6
             WHERE id = ?7",
            params![
                item.title,
                item.due_date,
                item.due_time,
                item.note,
                item.priority.as_str(),
                item.updated_at,
                item.id
            ],
        )?;
        Ok(changed)
    }

    pub fn update_status(
        conn: &Connection,
        id: &str,
        status: ItemStatus,
        updated_at: i64,
    ) -> AppResult<usize> {
        let changed = conn.execute(
            "UPDATE items SET status = ?1, updated_at = ?2 WHERE id = ?3",
            params![status.as_str(), updated_at, id],
        )?;
        Ok(changed)
    }

    pub fn delete(conn: &Connection, id: &str) -> AppResult<usize> {
        let changed = conn.execute("DELETE FROM items WHERE id = ?1", params![id])?;
        Ok(changed)
    }
}

fn map_row(row: &Row) -> rusqlite::Result<Item> {
    let status: String = row.get("status")?;
    let status = ItemStatus::parse(&status).ok_or_else(|| unknown_value("status", &status))?;
    let priority: String = row.get("priority")?;
    let priority =
        Priority::parse(&priority).ok_or_else(|| unknown_value("priority", &priority))?;

    Ok(Item {
        id: row.get("id")?,
        title: row.get("title")?,
        status,
        due_date: row.get("due_date")?,
        due_time: row.get("due_time")?,
        note: row.get("note")?,
        priority,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

fn unknown_value(column: &str, value: &str) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(
        0,
        rusqlite::types::Type::Text,
        format!("unknown item {column}: {value}").into(),
    )
}
