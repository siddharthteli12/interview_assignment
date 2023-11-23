use crate::{primitives::*, service::*, AppState};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to home")
}

#[post("/create_user")]
async fn create_user(db: web::Data<AppState>, user_info: web::Query<CreateUser>) -> impl Responder {
    match create_user_in_db(&db.db, user_info.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(json!({"code": 200, "status": "success","data": user})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"code": 404, "status": "error","message": e.to_string()})),
    }
}

#[get("/read_task/{task_id}")]
async fn read_task(db: web::Data<AppState>, path: web::Path<TaskId>) -> impl Responder {
    let task_id = path.into_inner();
    match read_task_from_db(&db.db, task_id).await {
        Ok(task) => HttpResponse::Ok().json(json!({"code": 200, "status": "success","data": task})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"code": 404, "status": "error","message": e.to_string()})),
    }
}

#[post("/add_task")]
async fn add_task(
    db: web::Data<AppState>,
    user_info: web::Query<CreateUserTask>,
) -> impl Responder {
    match create_task_in_db(&db.db, user_info.into_inner()).await {
        Ok(task) => HttpResponse::Ok().json(json!({"code": 200, "status": "success","data": task})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"code": 404, "status": "error","message": e.to_string()})),
    }
}

#[put("/update_task")]
async fn update_task(
    db: web::Data<AppState>,
    user_info: web::Query<UpdateUserTask>,
) -> impl Responder {
    match update_task_in_db(&db.db, user_info.into_inner()).await {
        Ok(task) => HttpResponse::Ok().json(json!({"code": 200, "status": "success","data": task})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"code": 404, "status": "error","message": e.to_string()})),
    }
}

#[delete("/delete_task/{task_id}")]
async fn delete_task(db: web::Data<AppState>, path: web::Path<TaskId>) -> impl Responder {
    let task_id = path.into_inner();
    match delete_task_from_db(&db.db, task_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"code": 200, "status": "success","data":task_id})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"code": 404, "status": "error","message": e.to_string()})),
    }
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