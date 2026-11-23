mod commands;
mod db;
mod dto;
mod error;
mod models;
mod repositories;
mod services;
mod state;
mod utils;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            app.manage(state::AppState::init(&data_dir)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::items::list_items,
            commands::items::create_item,
            commands::items::update_item,
            commands::items::set_item_status,
            commands::items::delete_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
