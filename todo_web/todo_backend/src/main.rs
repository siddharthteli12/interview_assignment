mod controller;
mod model;
mod primitives;
mod service;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use primitives::*;
use sqlx::{postgres::PgPool, Pool, Postgres};
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DB URL is not set");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Unable to connect to db server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(controller::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
