use tauri::State;

use crate::dto::project::{CreateProjectRequest, UpdateProjectRequest};
use crate::error::AppError;
use crate::models::item::Item;
use crate::models::project::{Project, ProjectStatus, ProjectSummary};
use crate::state::AppState;

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<ProjectSummary>, AppError> {
    state.projects.list()
}

#[tauri::command]
pub async fn get_project(state: State<'_, AppState>, id: String) -> Result<Project, AppError> {
    state.projects.get(&id)
}

#[tauri::command]
pub async fn list_project_tasks(
    state: State<'_, AppState>,
    id: String,
) -> Result<Vec<Item>, AppError> {
    state.projects.list_tasks(&id)
}

#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    request: CreateProjectRequest,
) -> Result<Project, AppError> {
    state.projects.create(request)
}

#[tauri::command]
pub async fn turn_task_into_project(
    state: State<'_, AppState>,
    item_id: String,
) -> Result<Project, AppError> {
    state.projects.create_from_task(&item_id)
}

#[tauri::command]
pub async fn update_project(
    state: State<'_, AppState>,
    id: String,
    request: UpdateProjectRequest,
) -> Result<Project, AppError> {
    state.projects.update(&id, request)
}

#[tauri::command]
pub async fn set_project_status(
    state: State<'_, AppState>,
    id: String,
    status: ProjectStatus,
) -> Result<Project, AppError> {
    state.projects.set_status(&id, status)
}

#[tauri::command]
pub async fn delete_project(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    state.projects.delete(&id)
}
