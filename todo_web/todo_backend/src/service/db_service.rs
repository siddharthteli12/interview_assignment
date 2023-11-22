use crate::{
    model::{TaskModel, UserModel},
    CreateUser, CreateUserTask, Pool, Postgres, UpdateUserTask,
};
use chrono::prelude::*;

pub async fn read_task(db: Pool<Postgres>, task_id: i32) -> Result<TaskModel, sqlx::Error> {
    sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks where task_id = $1",
        task_id.to_owned()
    )
    .fetch_one(&db)
    .await
}

pub async fn create_user(
    db: Pool<Postgres>,
    user_info: CreateUser,
) -> Result<UserModel, sqlx::Error> {
    sqlx::query_as!(
        UserModel,
        "INSERT INTO users (first_name, second_name) VALUES ($1, $2) RETURNING *",
        user_info.first_name.to_owned(),
        user_info.second_name.to_owned(),
    )
    .fetch_one(&db)
    .await
}

pub async fn create_task(
    db: Pool<Postgres>,
    user_info: CreateUserTask,
) -> Result<TaskModel, sqlx::Error> {
    sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (user_id, task_description) VALUES ($1, $2) RETURNING *",
        user_info.user_id.to_owned(),
        user_info.task_des.to_owned(),
    )
    .fetch_one(&db)
    .await
}

pub async fn update_task(
    db: Pool<Postgres>,
    user_info: UpdateUserTask,
) -> Result<TaskModel, sqlx::Error> {
    let query_result = sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks where task_id = $1",
        user_info.task_id.to_owned()
    )
    .fetch_one(&db)
    .await;
    let now = Utc::now();
    if query_result.is_err() {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as!(
        TaskModel,
        "UPDATE tasks SET task_description = $1, updated_at = $2 WHERE task_id = $3 RETURNING *",
        user_info.task_description,
        now,
        user_info.task_id,
    )
    .fetch_one(&db)
    .await
}
