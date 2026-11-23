use tauri::State;

use crate::dto::item::{CreateItemRequest, UpdateItemRequest};
use crate::error::AppError;
use crate::models::item::{Item, ItemStatus, TodayView};
use crate::state::AppState;

#[tauri::command]
pub async fn list_items(state: State<'_, AppState>) -> Result<Vec<Item>, AppError> {
    state.items.list()
}

#[tauri::command]
pub async fn list_today(state: State<'_, AppState>) -> Result<TodayView, AppError> {
    state.items.today(chrono::Local::now().date_naive())
}

#[tauri::command]
pub async fn create_item(
    state: State<'_, AppState>,
    request: CreateItemRequest,
) -> Result<Item, AppError> {
    state.items.create(request)
}

#[tauri::command]
pub async fn update_item(
    state: State<'_, AppState>,
    id: String,
    request: UpdateItemRequest,
) -> Result<Item, AppError> {
    state.items.update(&id, request)
}

#[tauri::command]
pub async fn set_item_status(
    state: State<'_, AppState>,
    id: String,
    status: ItemStatus,
) -> Result<Item, AppError> {
    state.items.set_status(&id, status)
}

#[tauri::command]
pub async fn delete_item(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    state.items.delete(&id)
}
