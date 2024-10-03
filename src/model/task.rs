#[derive(Serialize)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed
}

