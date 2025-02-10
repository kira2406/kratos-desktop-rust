// use actix::SyncArbiter;
use actix_web::{get, http::StatusCode, web::{self, Data, Json, Path}, App, HttpResponse, HttpServer, Responder};
// use actix_cors::Cors;
use actix_web::middleware::Logger;
// use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use dotenv::dotenv;
use serde::Serialize;
use serde_json::json;

mod db;
mod services;
mod routes;
use routes::*;


// use db::{establish_pool, AppState, DbActor};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let pool:Pool<ConnectionManager<PgConnection>> = establish_pool();
    // let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
        // Enable logger middleware
        .wrap(Logger::default())
        // Default route to indicate that the server is running
        .service(hello_user)
        // .configure(routes::init_routes)
        .service(server_response)
        .service(create_new_user)
   
    })
    .bind(("127.0.0.1", 8080))? // Bind the server to localhost:8080
    .run()
    .await
}
