use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::error::AppResult;
use crate::models::item::{Item, ItemStatus};

const COLUMNS: &str = "id, title, status, created_at, updated_at";

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
            "INSERT INTO items (id, title, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                item.id,
                item.title,
                item.status.as_str(),
                item.created_at,
                item.updated_at
            ],
        )?;
        Ok(())
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
    let status = ItemStatus::parse(&status).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            2,
            rusqlite::types::Type::Text,
            format!("unknown item status: {status}").into(),
        )
    })?;

    Ok(Item {
        id: row.get("id")?,
        title: row.get("title")?,
        status,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}
