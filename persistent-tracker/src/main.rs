
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    status: TaskStatus,
}


impl Task {
    // A constructor funcion . 'Self' means task
    fn new (name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: TaskStatus::Todo,
        }
    }

    fn mark_done(&mut self) {
        // self.is_done = true;
        self.status = TaskStatus::Done;
    }
}



// Telling Rust main can fall and return an IO error
fn main() -> Result<(), std::io::Error> {
    
    // let file_result = std::fs::read_to_string("LOGS.md");
   

    // match file_result {
    //     Ok(content) => println!("File contents: {}", content),
    //     Err(error) => println!("Oops, failed to read file: {}", error),
    // }

    //  let mut task_list: Vec<Task> = Vec::new();

    // 1. We will try to read file but using match instead of ? because if it fails we just want an empty vec , not a crash!
    let mut task_list: Vec<Task> = match std::fs::read_to_string("tasks.json") {
        // If the file exist we parse the string into a Vec<Task>
        // .unwrap_or_else means =>If the JSON is corrupted just give me an empty Vec instead of panicking
        Ok(json_content) => serde_json::from_str(&json_content).unwrap_or_else(|_| Vec::new()),
        // If the file doesn't exist yet , just start with an empty Vec
        Err(_) => Vec::new(),
    };

    // let task1 = Task::new(String::from("Learning Vectors"), String::from("Understanding Vec in Rust"));
    // let mut task2 = Task::new(String::from("Lean match"), String::from("Use match with Enum"));

    // my_task.mark_done();
    // task2.status = TaskStatus::InProgress;

    // println!("Task: {}", my_task.name);
    // println!("{:#?}", my_task);
    // task_list.push(task1);
    // task_list.push(task2);

    println!("---- LOADED TASKS ----");
    for task in &task_list {
        let status_string = match task.status {
         TaskStatus::Todo => "🔴 To-Do",
         TaskStatus::InProgress => "🔵 In Progress",
         TaskStatus::Done => "✅ Done",
        };
        println!("[{}] {} - {}", status_string, task.name, task.description);
    }

    let json_string = serde_json::to_string_pretty(&task_list)?;
    std::fs::write("tasks.json", json_string)?;

    // Return Ok at the very end to signal the program finished succesfully
    Ok(())
    
    
}
