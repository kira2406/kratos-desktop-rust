use actix_web::{
    get,
    http::StatusCode,
    web::{Json, Path},
    App,
    HttpServer,
    HttpResponse,
    Responder
};
use serde_json::json;

#[get("/")]
pub async fn server_response() -> impl Responder {
    HttpResponse::Ok().json(json!({"message":"Kratos Backend Initialized!"}))
}