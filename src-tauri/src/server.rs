use actix_web::{web, App, HttpServer};
use sqlx::{sqlite::SqlitePool, Sqlite};

use crate::routes;

pub fn start() {
    // Setup database connection pool
    let pool = SqlitePool::connect("sqlite:./db/db.sqlite").expect("Failed to create pool");
    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            // Pass the database connection pool to all routes
            .data(pool.clone())
            // Configure routes
            .service(web::scope("/app").configure(routes::app_routes))
    })
    .bind("127.0.0.1:16850") // Bind to localhost:16850
    .expect("Failed to bind to address")
    .run()
    .expect("Failed to run server");
}
