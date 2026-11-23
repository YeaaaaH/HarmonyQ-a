use std::thread;
use std::time::Duration;

use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::models::item::Item;
use crate::state::AppState;

const CHECK_EVERY: Duration = Duration::from_secs(20);

/// Checks for due reminders in the background for as long as the app runs (including hidden in the tray).
pub fn start(app: AppHandle) {
    thread::spawn(move || loop {
        notify_due(&app);
        thread::sleep(CHECK_EVERY);
    });
}

fn notify_due(app: &AppHandle) {
    let state = app.state::<AppState>();
    let due = match state
        .items
        .take_due_reminders(chrono::Local::now().naive_local())
    {
        Ok(due) => due,
        Err(err) => {
            eprintln!("reminders: {err}");
            return;
        }
    };
    for item in due {
        let shown = app
            .notification()
            .builder()
            .title(&item.title)
            .body(body(&item))
            .show();
        if let Err(err) = shown {
            eprintln!("reminders: {err}");
        }
    }
}

fn body(item: &Item) -> String {
    let time = item.due_time.as_deref().unwrap_or_default();
    match item.note.lines().next() {
        Some(line) => format!("{time} · {line}"),
        None => time.to_string(),
    }
}
