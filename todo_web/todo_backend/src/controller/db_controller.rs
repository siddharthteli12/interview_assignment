use crate::model::*;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to home")
}

#[get("/read_task/{task_id}")]
async fn read_task(path: web::Path<TaskId>) -> impl Responder {
    let task_id = path.into_inner();
    HttpResponse::Ok().body("Succesful")
}

#[post("/add_task")]
async fn add_task(user_info: web::Query<CreateUserTask>) -> impl Responder {
    println!("User info {:?}", user_info);
    HttpResponse::Ok().body("Task added succefully")
}

#[put("/update_task")]
async fn update_task(user_info: web::Query<UpdateUserTask>) -> impl Responder {
    println!("User info {:?}", user_info);
    HttpResponse::Ok().body("Task updated succefully")
}

#[delete("/delete_task/{task_id}")]
async fn delete_task(path: web::Path<TaskId>) -> impl Responder {
    HttpResponse::Ok().body("Task deleted succefully")
}
