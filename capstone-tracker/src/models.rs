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


pub struct TaskBuilder {
    id: u64,
    name: Option<String>,
    description: Option<String>,
}

// Methods that take ownership of self , modify it and return it back

impl TaskBuilder {
    pub fn name(mut self, name: String) -> Self
    {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Task {
        Task {
            id: TaskId(self.id),
            name: self.name.unwrap_or_else(|| String::from("Untitled")),
            description: self.description.unwrap_or_else(|| String::from("NO description")),
            status: TaskStatus::Todo,
        }
    }
}



impl Task {
    // pub fn new (id: u64, name: String, description: String) -> Self {
    //     Self {
    //         id: TaskId(id), // we wrapped the raw u64 inside our TaskId stuct!
    //         name,
    //         description,
    //         status: TaskStatus::Todo,
    //     }
    // }

    pub fn builder(id: u64) -> TaskBuilder {
        TaskBuilder {
            id,
            name: None,
            description: None,
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
        // let task = Task::new(1,String::from("Test"), String::from("Testing"));
        let task = Task::builder(1).name("Test".to_string()).description("Testing".to_string()).build();
        // assert_eq! => check If the two values are exactly the same 
        assert_eq!(task.name, "Test");

        // We have to use matches! macro because we can't easily compare enums unless we add #[derive(PartialEq)] to them!
        assert!(matches!(task.status, TaskStatus::Todo));
    }

    #[test]
    fn test_task_mark_done() {
        // let mut task = Task::new(1,String::from("Test"), String::from("Testing"));
        let mut task = Task::builder(1).name("Test".to_string()).description("Testing".to_string()).build();
        task.mark_done();
        assert!(matches!(task.status, TaskStatus::Done));
    }
}