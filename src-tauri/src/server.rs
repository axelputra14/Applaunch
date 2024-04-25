use actix_web::{web, App, HttpServer};

use crate::routes;

pub fn start() {
    // Start Actix web server
    HttpServer::new(|| {
        App::new()
            // Configure routes
            .service(web::scope("/app").configure(routes::app_routes))
    })
    .bind("127.0.0.1:16850") // Bind to localhost:8080
    .expect("Failed to bind to address")
    .run();
    // .expect("Failed to run server");
}
