#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use active_win_pos_rs::get_active_window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    match get_active_window() {
        Ok(active_window) => {
            // println!("active window: {:#?}", active_window);
            format!("Hello, {:#?}", active_window)
        }
        Err(()) => {
            // println!("error occurred while getting the active window");
            format!("Error while fetching window data")
        }
    }
    // format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
