use serde::Deserialize;

pub type UserId = i32;
pub type TaskId = i32;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub second_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserTask {
    pub user_id: UserId,
    pub task_des: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserTask {
    pub user_id: UserId,
    pub task_id: TaskId,
    pub task_description: String,
}
