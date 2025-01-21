// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use std::process::Command;

mod db;


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


async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|_app| {
            db::init();   
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            launch_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
