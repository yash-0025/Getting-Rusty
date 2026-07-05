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


#[cfg(test)]
mod tests {
    // Bring everythign from the parent module (models.rs) into this test module
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(String::from("Test"), String::from("Testing"));
        // assert_eq! => check If the two values are exactly the same 
        assert_eq!(task.name, "Test");

        // We have to use matches! macro because we can't easily compare enums unless we add #[derive(PartialEq)] to them!
        assert!(matches!(task.status, TaskStatus::Todo));
    }

    #[test]
    fn test_task_mark_done() {
        let mut task = Task::new(String::from("Test"), String::from("Testing"));
        task.mark_done();
        assert!(matches!(task.status, TaskStatus::Done));
    }
}