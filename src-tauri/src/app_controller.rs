use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use chrono::NaiveDate;

#[derive(Debug, sqlx::FromRow)]
pub struct App {
    pub id: i32,
    pub title: String,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub exe_dir: String,
    pub img_dir: String,
    pub bg_dir: String,
    pub desc: Option<String>,
    pub lang: String,
    pub rel_date: Option<chrono::NaiveDate>,
}

pub async fn list_app(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as::<_, App>("SELECT * FROM apps ORDER BY title ASC")
        .fetch_all(&**pool)
        .await;

    match result {
        Ok(apps) => HttpResponse::Ok().json(apps),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn add_app(pool: web::Data<SqlitePool>) -> impl Responder {
    // Implement logic to add a new app to the database
    // You can access the pool using `pool.get_ref()`

    HttpResponse::Ok().body("Add app request received")
}

pub async fn update_app(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    // Implement logic to update an existing app in the database
    // You can access the pool using `pool.get_ref()`
    let id = path.into_inner();

    HttpResponse::Ok().body(format!("Update app request received for id {}", id))
}

pub async fn list_one_app(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    // Implement logic to fetch a single app from the database by ID
    // You can access the pool using `pool.get_ref()`
    let id = path.into_inner();

    HttpResponse::Ok().body(format!("List app request received for id {}", id))
}

pub async fn delete_app(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    // Implement logic to delete an app from the database by ID
    // You can access the pool using `pool.get_ref()`
    let id = path.into_inner();

    HttpResponse::Ok().body(format!("Delete app request received for id {}", id))
}
