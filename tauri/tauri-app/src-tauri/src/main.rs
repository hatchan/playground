// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
async fn greet(first_name: &str, last_name: Option<&str>, bla: &str) -> Result<String, ()> {
    if let Some(last_name) = last_name {
        Ok(format!(
            "Hello, {first_name} {last_name}! You've been greeted from Rust!"
        ))
    } else {
        Ok(format!("Hello {first_name}, I don't know your last name!"))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
