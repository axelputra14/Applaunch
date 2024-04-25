use actix_web::web;

use crate::app_controller;

pub fn app_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(app_controller::list_app))
            .route("/add", web::post().to(app_controller::add_app))
            .route("/edit/{id}", web::patch().to(app_controller::update_app))
            .route("/{id}", web::get().to(app_controller::list_one_app))
            .route("/{id}", web::delete().to(app_controller::delete_app)),
    );
}
