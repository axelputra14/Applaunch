// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::process::Command;
use tauri::api::dialog::blocking::FileDialogBuilder;
use std::env;
use std::path::Path;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use tokio::runtime::Runtime;
use std::thread;
use tokio::fs;

#[derive(Serialize, Deserialize, FromRow)]
struct AppData {
    id: Option<i64>,
    title: String,
    developer: String,
    exeDir: String,
    imgDir: String,
}


async fn get_apps(pool: web::Data<SqlitePool>) -> impl Responder {
    let apps = sqlx::query_as::<_, AppData>("SELECT * FROM App")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_else(|_| vec![]); // Simply return an empty list on error
    
    HttpResponse::Ok().json(apps)
}

async fn add_app(pool: web::Data<SqlitePool>, new_app: web::Json<AppData>) -> impl Responder {
    let result = sqlx::query("INSERT INTO App (title, developer, exeDir, imgDir) VALUES (?, ?, ?, ?)")
        .bind(&new_app.title)
        .bind(&new_app.developer)
        .bind(&new_app.exeDir)
        .bind(&new_app.imgDir)
        .execute(pool.get_ref())
        .await;
    
    match result {
        Ok(_) => HttpResponse::Created().json(new_app.0),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_app(pool: web::Data<SqlitePool>, app_id: web::Path<i64>, updated_app: web::Json<AppData>) -> impl Responder {
    let result = sqlx::query("UPDATE App SET title = ?, developer = ?, exeDir = ?, imgDir = ? WHERE id = ?")
        .bind(&updated_app.title)
        .bind(&updated_app.developer)
        .bind(&updated_app.exeDir)
        .bind(&updated_app.imgDir)
        .bind(app_id.into_inner())
        .execute(pool.get_ref())
        .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json(updated_app.0),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_app(pool: web::Data<SqlitePool>, app_id: web::Path<i64>) -> impl Responder {
    let result = sqlx::query("DELETE FROM App WHERE id = ?")
        .bind(app_id.into_inner())
        .execute(pool.get_ref())
        .await;
    
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

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

async fn server_start() -> std::io::Result<()> {
    let database_url = "sqlite://./db/db.sqlite";
    let database_file = "./db/db.sqlite";

    // Ensure the directory exists
    std::fs::create_dir_all("./db").expect("Failed to create database directory");

    // Check if the database file exists
    let pool = if !Path::new(database_file).exists() {
        println!("Database file '{}' not found, initializing...", database_file);

        // Read setup.sql file
        let setup_sql = fs::read_to_string("setup.sql").await.expect("Failed to read setup.sql");

        // Connect to SQLite database
        let pool = SqlitePool::connect(database_url)
            .await
            .expect("Failed to create pool.");

        // Execute setup.sql script
        sqlx::query(&setup_sql)
            .execute(&pool)
            .await
            .expect("Failed to execute setup.sql");

        println!("Database initialized successfully.");
        pool
    } else {
        SqlitePool::connect(database_url)
            .await
            .expect("Failed to create pool.")
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Pass pool as application data
            .route("/app", web::get().to(get_apps))
            .route("/app/add", web::post().to(add_app))
            .route("/app/{id}", web::patch().to(update_app))
            .route("/app/{id}", web::delete().to(delete_app))
    })
    .bind("127.0.0.1:16850")?
    .run()
    .await
}

async fn run_server() {
    server_start().await.expect("Failed to start server");
}

fn main() {
    // Create a Tokio runtime
    let runtime = Runtime::new().expect("Failed to create runtime");

    // Start the Actix server in a separate thread
    thread::spawn(move || {
        runtime.block_on(async {
            run_server().await; // No unwrap, just await
        });
    });

    // Run the Tauri application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_app, select_exe, select_img, select_bg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
