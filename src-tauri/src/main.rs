// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::handle_drag_and_drop_files,
            cmd::handle_rename_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
