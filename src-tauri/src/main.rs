// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// 
use std::process::Command;
use tauri::{
    // api::process::Command,
    Manager,
};

#[tauri::command]
fn launch_app(app_location: &str) {

    // Create a new Command
    let mut cmd = Command::new(app_location);

    // For arguments cmd.arg("arg1").arg("arg2");

    // Execute the command
    match cmd.spawn() {
        Ok(_) => println!("Successfully launched the executable"),
        Err(e) => eprintln!("Failed to launch the executable: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
    .setup(|app| {
      let _window = app.get_window("main").unwrap();


      Ok(())
    })
    .invoke_handler(tauri::generate_handler![launch_app])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}