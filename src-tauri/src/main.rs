
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusty_sheet_core::Choice;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn getCharacterOptions() -> Vec<Choice>
{
    vec![]
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, getCharacterOptions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
