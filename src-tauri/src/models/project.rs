use serde::{Deserialize, Serialize};

use crate::models::item::Item;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProjectStatus {
    Active,
    Paused,
    Done,
}

impl ProjectStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ProjectStatus::Active => "active",
            ProjectStatus::Paused => "paused",
            ProjectStatus::Done => "done",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "active" => Some(ProjectStatus::Active),
            "paused" => Some(ProjectStatus::Paused),
            "done" => Some(ProjectStatus::Done),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    /// "Done when ...", may be empty.
    pub outcome: String,
    pub status: ProjectStatus,
    /// "Where I left off".
    pub resume_note: String,
    /// 0-359, picked from the name at creation and kept on rename.
    pub hue: u16,
    pub created_at: i64,
    pub updated_at: i64,
    pub completed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSummary {
    #[serde(flatten)]
    pub project: Project,
    /// First open task by date, time, priority, age, but only if it has a date or is important:
    /// picking the oldest of undated tasks would look like a choice that was never made.
    /// `None` with `open_count == 0` means "needs a next step".
    pub next_step: Option<Item>,
    pub open_count: usize,
    pub done_count: usize,
    /// Latest change to the project or any of its tasks.
    pub last_activity_at: i64,
}
