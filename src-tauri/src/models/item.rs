use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemStatus {
    Open,
    Done,
}

impl ItemStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ItemStatus::Open => "open",
            ItemStatus::Done => "done",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "open" => Some(ItemStatus::Open),
            "done" => Some(ItemStatus::Done),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
    #[default]
    Normal,
    High,
}

impl Priority {
    pub fn as_str(self) -> &'static str {
        match self {
            Priority::Normal => "normal",
            Priority::High => "high",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "normal" => Some(Priority::Normal),
            "high" => Some(Priority::High),
            _ => None,
        }
    }
}

impl Item {
    /// Order of tasks on the same day: timed before untimed, by time, important first, oldest first.
    pub fn within_day_key(&self) -> (bool, Option<&str>, bool, i64) {
        (
            self.due_time.is_none(),
            self.due_time.as_deref(),
            self.priority != Priority::High,
            self.created_at,
        )
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodayView {
    /// The local date the view was built for, `YYYY-MM-DD`.
    pub date: String,
    /// Open tasks from earlier days.
    pub overdue: Vec<Item>,
    /// Tasks for `date`, open ones first.
    pub today: Vec<Item>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub title: String,
    pub status: ItemStatus,
    /// Calendar day in the user's local time, `YYYY-MM-DD`.
    pub due_date: Option<String>,
    /// Local wall-clock time, `HH:MM`. Only set together with `due_date`.
    pub due_time: Option<String>,
    pub note: String,
    pub priority: Priority,
    pub project_id: Option<String>,
    /// Set when the task is ticked, cleared when unticked. Tasks done before projects existed have none.
    pub completed_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}
