mod commands;
mod db;
mod dto;
mod error;
mod models;
mod reminders;
mod repositories;
mod services;
mod state;
mod tray;
mod utils;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        // Must be the first plugin: a second launch only focuses the running instance.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::show_main_window(app)
        }))
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            app.manage(state::AppState::init(&data_dir)?);
            tray::init(app.handle())?;
            reminders::start(app.handle().clone());
            Ok(())
        })
        .on_window_event(tray::hide_on_close)
        .invoke_handler(tauri::generate_handler![
            commands::items::list_items,
            commands::items::list_today,
            commands::items::create_item,
            commands::items::update_item,
            commands::items::set_item_status,
            commands::items::set_item_project,
            commands::items::delete_item,
            commands::projects::list_projects,
            commands::projects::get_project,
            commands::projects::list_project_tasks,
            commands::projects::create_project,
            commands::projects::turn_task_into_project,
            commands::projects::update_project,
            commands::projects::set_project_status,
            commands::projects::delete_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
