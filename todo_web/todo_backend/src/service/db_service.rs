// Internal dependencies
use crate::{
    model::{TaskModel, UserModel},
    CreateUser, CreateUserTask, Pool, Postgres, UpdateUserTask,
};

// External dependencies
use chrono::prelude::*;
use sqlx::postgres::PgQueryResult;

/// Reads given task id from db
///
/// # Arguments
///
/// * `db` - A reference to the database connection pool.
/// * `task_id` - The ID of the task to be retrieved.
///
/// # Returns
///
/// Returns a Result containing a TaskModel if successful or error.
pub async fn read_task_from_db(
    db: &Pool<Postgres>,
    task_id: i32,
) -> Result<TaskModel, sqlx::Error> {
    sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks where task_id = $1",
        task_id.to_owned()
    )
    .fetch_one(db)
    .await
}

/// Creates new user with unique mail.
///
/// # Arguments
///
/// * `db` - A reference to the database connection pool.
/// * `user_info` - User info like mail, first/ second name.
///
/// # Returns
///
/// Returns a Result containing a UserModel if successful or error.
pub async fn create_user_in_db(
    db: &Pool<Postgres>,
    user_info: CreateUser,
) -> Result<UserModel, sqlx::Error> {
    sqlx::query_as!(
        UserModel,
        "INSERT INTO users (user_mail, first_name, second_name) VALUES ($1, $2, $3) RETURNING *",
        user_info.user_mail.to_owned(),
        user_info.first_name.to_owned(),
        user_info.second_name.to_owned(),
    )
    .fetch_one(db)
    .await
}

/// Creates new task for a given user mail id.
///
/// # Arguments
///
/// * `db` - A reference to the database connection pool.
/// * `task_info` - Task info like description, user mail.
///
/// # Returns
///
/// Returns a Result containing a TaskModel if successful or error.
pub async fn create_task_in_db(
    db: &Pool<Postgres>,
    task_info: CreateUserTask,
) -> Result<TaskModel, sqlx::Error> {
    sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (user_mail, task_description) VALUES ($1, $2) RETURNING *",
        task_info.user_mail.to_owned(),
        task_info.task_des.to_owned(),
    )
    .fetch_one(db)
    .await
}

/// Updates existing task for a given user mail id.
///
/// # Arguments
///
/// * `db` - A reference to the database connection pool.
/// * `task_info` - New task descrition.
///
/// # Returns
///
/// Returns a Result containing a TaskModel if successful or error.
pub async fn update_task_in_db(
    db: &Pool<Postgres>,
    task_info: UpdateUserTask,
) -> Result<TaskModel, sqlx::Error> {
    let query_result = sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks where task_id = $1",
        task_info.task_id.to_owned()
    )
    .fetch_one(db)
    .await;
    let now = Utc::now();
    if query_result.is_err() {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as!(
        TaskModel,
        "UPDATE tasks SET task_description = $1, updated_at = $2 WHERE task_id = $3 RETURNING *",
        task_info.task_description,
        now,
        task_info.task_id,
    )
    .fetch_one(db)
    .await
}

/// Deletes existing task.
///
/// # Arguments
///
/// * `db` - A reference to the database connection pool.
/// * `task_id` - Task id to delete from db.
///
/// # Returns
///
/// Returns a Result containing a Pg query result if successful or error.
pub async fn delete_task_from_db(
    db: &Pool<Postgres>,
    task_id: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_as!(TaskModel, "DELETE FROM tasks WHERE task_id = $1", task_id)
        .execute(db)
        .await
}
