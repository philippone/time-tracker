use active_win_pos_rs::get_active_window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // format!("Hello, {}! You've been greeted from Rust!", name)

    match get_active_window() {
        Ok(active_window) => {
            // println!("active window: {:#?}", active_window);
            format!("active window: {:#?}", active_window)
        }
        Err(()) => {
            // println!("error occurred while getting the active window");
            format!("error occurred while getting the active window")
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
