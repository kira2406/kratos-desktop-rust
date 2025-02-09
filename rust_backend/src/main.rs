use actix::SyncArbiter;
use actix_web::{middleware, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use dotenv::dotenv;
use serde_json::json;
use std::env;

mod routes;
mod db;

use db::{establish_pool, AppState, DbActor};

async fn server_response() -> impl Responder {
    HttpResponse::Ok().json(json!({"message":"Kratos Backend Initialized!"}))
}

async fn kush_response() -> impl Responder {
    HttpResponse::Ok().json(json!({"message":"Kush here!"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool:Pool<ConnectionManager<PgConnection>> = establish_pool();
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(AppState{db:db_addr.clone()}))
        // Enable logger middleware
        .wrap(Logger::default())
        // Set up CORS
        .wrap(Cors::default())
        // Default route to indicate that the server is running
        .route("/kush", web::get().to(kush_response))
        // .configure(routes::init_routes)
        .default_service(web::route().to(server_response))
   
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost:8080
    .run()
    .await
}
