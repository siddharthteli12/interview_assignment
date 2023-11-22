use serde::Deserialize;

pub type UserId = u64;
pub type TaskId = u64;

#[derive(Debug, Deserialize)]
pub struct CreateUserTask {
    pub user_id: UserId,
    pub task_des: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserTask {
    pub user_id: UserId,
    pub task_id: TaskId,
    pub task_des: String,
}
