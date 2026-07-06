use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskId(pub u64);


impl Task {
    pub fn new (id: u64, name: String, description: String) -> Self {
        Self {
            id: TaskId(id), // we wrapped the raw u64 inside our TaskId stuct!
            name,
            description,
            status: TaskStatus::Todo,
        }
    }

    pub fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
    }
}


#[cfg(test)]
mod tests {
    // Bring everythign from the parent module (models.rs) into this test module
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1,String::from("Test"), String::from("Testing"));
        // assert_eq! => check If the two values are exactly the same 
        assert_eq!(task.name, "Test");

        // We have to use matches! macro because we can't easily compare enums unless we add #[derive(PartialEq)] to them!
        assert!(matches!(task.status, TaskStatus::Todo));
    }

    #[test]
    fn test_task_mark_done() {
        let mut task = Task::new(1,String::from("Test"), String::from("Testing"));
        task.mark_done();
        assert!(matches!(task.status, TaskStatus::Done));
    }
}