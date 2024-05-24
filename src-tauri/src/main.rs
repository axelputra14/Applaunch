// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// 
use std::process::Command;
use tauri::api::dialog::blocking::FileDialogBuilder;

#[tauri::command]
fn launch_app(exeDir: &str) {
    // Create a new Command
    let mut cmd = Command::new(exeDir);

    // Execute the command
    match cmd.spawn() {
        Ok(_) => println!("Successfully launched the executable"),
        Err(e) => eprintln!("Failed to launch the executable: {}", e),
    }
}

#[tauri::command]
fn select_file() -> Result<String, String> {

    let file_chosen = FileDialogBuilder::new().set_directory("").pick_file();
    
    match file_chosen {
        Some(path) => {
            // Convert the file path to a string and return it
            match path.to_str() {
                Some(path_str) => Ok(path_str.to_string()),
                None => Err("Failed to convert path to string".to_string()),
            }
        }
        None => Err("No file selected".to_string()),
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_app, select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
