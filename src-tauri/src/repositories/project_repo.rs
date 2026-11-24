use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::error::AppResult;
use crate::models::project::{Project, ProjectStatus};

const COLUMNS: &str =
    "id, name, outcome, status, resume_note, hue, created_at, updated_at, completed_at";

pub struct ProjectRepository;

impl ProjectRepository {
    pub fn list(conn: &Connection) -> AppResult<Vec<Project>> {
        let mut stmt = conn.prepare(&format!("SELECT {COLUMNS} FROM projects"))?;
        let projects = stmt
            .query_map([], map_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(projects)
    }

    pub fn find(conn: &Connection, id: &str) -> AppResult<Option<Project>> {
        let project = conn
            .query_row(
                &format!("SELECT {COLUMNS} FROM projects WHERE id = ?1"),
                params![id],
                map_row,
            )
            .optional()?;
        Ok(project)
    }

    pub fn insert(conn: &Connection, project: &Project) -> AppResult<()> {
        conn.execute(
            &format!(
                "INSERT INTO projects ({COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
            ),
            params![
                project.id,
                project.name,
                project.outcome,
                project.status.as_str(),
                project.resume_note,
                project.hue,
                project.created_at,
                project.updated_at,
                project.completed_at
            ],
        )?;
        Ok(())
    }

    pub fn update_details(conn: &Connection, project: &Project) -> AppResult<usize> {
        let changed = conn.execute(
            "UPDATE projects SET name = ?1, outcome = ?2, resume_note = ?3, updated_at = ?4
             WHERE id = ?5",
            params![
                project.name,
                project.outcome,
                project.resume_note,
                project.updated_at,
                project.id
            ],
        )?;
        Ok(changed)
    }

    pub fn update_status(conn: &Connection, project: &Project) -> AppResult<usize> {
        let changed = conn.execute(
            "UPDATE projects SET status = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                project.status.as_str(),
                project.completed_at,
                project.updated_at,
                project.id
            ],
        )?;
        Ok(changed)
    }

    pub fn delete(conn: &Connection, id: &str) -> AppResult<usize> {
        let changed = conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(changed)
    }
}

fn map_row(row: &Row) -> rusqlite::Result<Project> {
    let status: String = row.get("status")?;
    let status = ProjectStatus::parse(&status).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            rusqlite::types::Type::Text,
            format!("unknown project status: {status}").into(),
        )
    })?;

    Ok(Project {
        id: row.get("id")?,
        name: row.get("name")?,
        outcome: row.get("outcome")?,
        status,
        resume_note: row.get("resume_note")?,
        hue: row.get("hue")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
        completed_at: row.get("completed_at")?,
    })
}
