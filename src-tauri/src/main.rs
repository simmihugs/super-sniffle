// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

#[tauri::command]
fn log_it(message: String) {
    println!("log: {}", message);
}

#[tauri::command]
fn echo_it(message: String) {
    println!("echo: {}", message);
}

#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    // Should handle errors instead of unwrapping here
    app_handle.db(|db| database::add_item(name, db)).unwrap();

    let items = app_handle.db(|db| database::get_all(db)).unwrap();

    let items_string = items.join(" | ");

    format!("Your name log: {}", items_string)
}

fn main() {
    tauri::Builder::default()
    //.invoke_handler(tauri::generate_handler![greet])
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![log_it, echo_it, greet])
	.setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
	.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
