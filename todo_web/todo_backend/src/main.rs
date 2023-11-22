use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};

mod model;
use model::*;

mod controller;
use controller::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(read_task)
            .service(add_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
