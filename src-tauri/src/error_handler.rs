use actix_web::{dev, web, HttpResponse};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::StatusCode;
use serde_json::json;

pub fn error_handler() -> ErrorHandlers<HttpResponse> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server_error)
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::BAD_REQUEST, bad_request)
        .handler(StatusCode::FORBIDDEN, forbidden)
}

fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<HttpResponse>> {
    let body = json!({"error": "Internal Server Error"});
    let response = HttpResponse::InternalServerError().json(body);
    Ok(ErrorHandlerResponse::Response(res.into_response(response)))
}

fn not_found<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<HttpResponse>> {
    let body = json!({"error": "Not Found"});
    let response = HttpResponse::NotFound().json(body);
    Ok(ErrorHandlerResponse::Response(res.into_response(response)))
}

fn bad_request<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<HttpResponse>> {
    let body = json!({"error": "Bad Request"});
    let response = HttpResponse::BadRequest().json(body);
    Ok(ErrorHandlerResponse::Response(res.into_response(response)))
}

fn forbidden<B>(res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<HttpResponse>> {
    let body = json!({"error": "Forbidden"});
    let response = HttpResponse::Forbidden().json(body);
    Ok(ErrorHandlerResponse::Response(res.into_response(response)))
}
