use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use actix::{Actor, Addr, SyncContext};
use std::env;
use actix_web::web;
use diesel::pg::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

// Establish a connection to the database
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct AppState {
    pub db:Addr<DbActor>
}

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor{
    type Context = SyncContext<Self>;
}

// Setup the DBPool
pub fn establish_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}



