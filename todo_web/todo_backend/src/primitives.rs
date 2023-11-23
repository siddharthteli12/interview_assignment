use serde::Deserialize;
pub type TaskId = i32;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub user_mail: String,
    pub first_name: String,
    pub second_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserTask {
    pub user_mail: String,
    pub task_des: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserTask {
    pub user_mail: String,
    pub task_id: TaskId,
    pub task_description: String,
}
