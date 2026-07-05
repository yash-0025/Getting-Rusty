use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String, 
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}


impl Task {
    pub fn new (name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: TaskStatus::Todo,
        }
    }

    pub fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
    }
}