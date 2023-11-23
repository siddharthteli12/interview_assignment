// Internal modules
mod controller;
mod model;
mod primitives;
mod service;

// External dependencies
use actix_web::{web, App, HttpServer};
use controller::*;
use dotenv::dotenv;
use primitives::*;
use sqlx::{postgres::PgPool, Pool, Postgres};

// Application state, connection for db pool
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variables.
    dotenv().ok();
    // Init the logger.
    env_logger::init();
    // Read Db url set in .env
    let db_url = std::env::var("DATABASE_URL").expect("DB URL is not set");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Unable to connect to db server");

    // Start actix server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(config)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
