#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    is_done: bool,
}

impl Task {
    // A constructor funcion . 'Self' means task
    fn new (name: String, description: String) -> Self {
        Self {
            name,
            description,
            is_done: false
        }
    }
}

fn main() {


    // let my_task = Task {
    //     name: String::from("Learning Structs"),
    //     description: String::from("Learning about grouping data in Rust"),
    //     is_done: false,
    // };

    let my_task = Task::new(
        String::from("Learning impl"),
        String::from("Creating task Constructor")
    );

    // println!("Task: {}", my_task.name);
    println!("{:#?}", my_task);
}
