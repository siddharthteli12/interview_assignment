use crate::{primitives::*, AppState};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to home")
}

#[post("/create_user")]
async fn create_user(db: web::Data<AppState>, user_info: web::Query<CreateUser>) -> impl Responder {
    println!("User info {:?}", user_info);
    HttpResponse::Ok().body("Task added succefully")
}

#[get("/read_task/{task_id}")]
async fn read_task(db: web::Data<AppState>, path: web::Path<TaskId>) -> impl Responder {
    let task_id = path.into_inner();
    HttpResponse::Ok().body("Succesful")
}

#[post("/add_task")]
async fn add_task(
    db: web::Data<AppState>,
    user_info: web::Query<CreateUserTask>,
) -> impl Responder {
    println!("User info {:?}", user_info);
    HttpResponse::Ok().body("Task added succefully")
}

#[put("/update_task")]
async fn update_task(
    db: web::Data<AppState>,
    user_info: web::Query<UpdateUserTask>,
) -> impl Responder {
    println!("User info {:?}", user_info);
    HttpResponse::Ok().body("Task updated succefully")
}

#[delete("/delete_task/{task_id}")]
async fn delete_task(db: web::Data<AppState>, path: web::Path<TaskId>) -> impl Responder {
    HttpResponse::Ok().body("Task deleted succefully")
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(home)
        .service(read_task)
        .service(add_task)
        .service(update_task)
        .service(delete_task);

    conf.service(scope);
}
