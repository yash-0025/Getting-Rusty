// We use "crate" to refer to the root of our project the models module
use crate::models::Task;

pub fn load_tasks() -> Vec<Task> {
    match std::fs::read_to_string("tasks.json") {
        Ok(json_content) => serde_json::from_str(&json_content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

pub fn save_tasks(task_list: &Vec<Task>) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string_pretty(task_list)?;
    std::fs::write("tasks.json", json_string)?;
    Ok(())

}