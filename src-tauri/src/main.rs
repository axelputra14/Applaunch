// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]

use std::process::Command;
use tauri::api::dialog::blocking::FileDialogBuilder;
use std::env;
// use std::path::PathBuf;

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
fn select_exe() -> Result<String, String> {

    let file_chosen = FileDialogBuilder::new().set_directory("").pick_file();
    
    match file_chosen {
        Some(path) => {
            // Convert the file path to a string and return it
            match path.to_str() {
                Some(path_str) => Ok(path_str.to_string()), // This is where the file path is returned if conversion succeeds
                None => Err("Failed to convert path to string".to_string()),
            }
        }
        None => Err("No file selected".to_string()),
    }

}

#[tauri::command]
fn select_img() -> Result<String, String> {
    // Specify the relative default directory
    let relative_directory = "C:\\Users\\Axel\\Documents\\covercatalog\\cover"; 

    // Get the current working directory
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;

    // Create the full path to the default directory
    let default_directory = current_dir.join(relative_directory);

    let file_chosen = FileDialogBuilder::new()
        .set_directory(default_directory)
        .pick_file();

    match file_chosen {
        Some(path) => {
            // Extract the file name from the path and return it
            match path.file_name() {
                Some(file_name) => match file_name.to_str() {
                    Some(file_name_str) => Ok(file_name_str.to_string()),
                    None => Err("Failed to convert file name to string".to_string()),
                },
                None => Err("Failed to extract file name".to_string()),
            }
        }
        None => Err("No file selected".to_string()), // This is where the error is returned if no file is selected
    }
}

#[tauri::command]
fn select_bg() -> Result<String, String> {
    // Specify the relative default directory
    let relative_directory = "C:\\Users\\Axel\\Documents\\covercatalog\\bg"; 

    // Get the current working directory
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;

    // Create the full path to the default directory
    let default_directory = current_dir.join(relative_directory);

    let file_chosen = FileDialogBuilder::new()
        .set_directory(default_directory)
        .pick_file();

    match file_chosen {
        Some(path) => {
            // Extract the file name from the path and return it
            match path.file_name() {
                Some(file_name) => match file_name.to_str() {
                    Some(file_name_str) => Ok(file_name_str.to_string()),
                    None => Err("Failed to convert file name to string".to_string()),
                },
                None => Err("Failed to extract file name".to_string()),
            }
        }
        None => Err("No file selected".to_string()), // This is where the error is returned if no file is selected
    }
}



fn main() {
    tauri::Builder::default()
    .setup(|app| {
  
        Ok(())
      })
        .invoke_handler(tauri::generate_handler![launch_app, select_exe, select_img, select_bg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
