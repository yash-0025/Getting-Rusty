
#[derive(Debug)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}


#[derive(Debug)]
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

fn main() {


    // let my_task = Task {
    //     name: String::from("Learning Structs"),
    //     description: String::from("Learning about grouping data in Rust"),
    //     is_done: false,
    // };

    // let mut my_task = Task::new(
    //     String::from("Learning impl"),
    //     String::from("Creating task Constructor")
    // );

    let mut task_list: Vec<Task> = Vec::new();

    let task1 = Task::new(String::from("Learning Vectors"), String::from("Understanding Vec in Rust"));
    let mut task2 = Task::new(String::from("Lean match"), String::from("Use match with Enum"));

    // my_task.mark_done();
    task2.status = TaskStatus::InProgress;

    // println!("Task: {}", my_task.name);
    // println!("{:#?}", my_task);
    task_list.push(task1);
    task_list.push(task2);


    for task in task_list {
        let status_string = match task.status {
            TaskStatus::Todo => "🔴 To-Do",
            TaskStatus::InProgress => "🔵 In Progress",
            TaskStatus::Done => "✅ Done",
        };

      println!("[{}] {} - {}", status_string, task.name, task.description);
    }
}
