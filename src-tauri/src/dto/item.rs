use serde::Deserialize;

use crate::models::item::Priority;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemRequest {
    pub title: String,
    #[serde(default)]
    pub due_date: Option<String>,
}

/// Full replacement of the editable fields; the UI always sends all of them.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItemRequest {
    pub title: String,
    pub due_date: Option<String>,
    pub due_time: Option<String>,
    pub note: String,
    pub priority: Priority,
}
