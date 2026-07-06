
use serde::{Serialize, Deserialize};
use clap::Parser;

mod models; // This tells rust to look up for a models.rs file
mod storage;
mod cli;

use models::{Task, TaskStatus};// This brings them into scope so we don't have to type models::Task
use cli::{Cli, Commands};


// Telling Rust main can fall and return an IO error
fn main() -> Result<(), std::io::Error> {

    let cli = Cli::parse();
    // println!("Parsed CLI : {:#?}", cli);
    
    let mut task_list = storage::load_tasks();

    match cli.command {
        Commands::Add {name, description} => {
            let next_id = (task_list.len() as u64) + 1;
            let new_task = Task::new(next_id, name, description);
            task_list.push(new_task);
            println!("✅ Task Added ");
        }
        Commands::List => {
            println!("---ALL TASKS---");
            for task in &task_list {
                let status_string = match task.status{
                    TaskStatus::Todo => "🔴 TO-DO",
                    TaskStatus::InProgress => "🔵 IN PROGRESS",
                    TaskStatus::Done => "✅ DONE",
                };

                println!("[ID: {}] [{}] {} - {}",task.id.0, status_string, task.name, task.description);
            }
        }
        Commands::Complete { id } => {
            let target_task = task_list.iter_mut().find(|task| task.id.0 == id);

            match target_task {
                Some(task) => {
                    task.mark_done();
                    println!("✅ Task {} marked as completed!", id);
                }
                None => {
                    println!("❌ Could not find a task with ID {}", id);
                }
            }
        }
    }

    storage::save_tasks(&task_list)?;
    // Return Ok at the very end to signal the program finished succesfully
    Ok(())
    
    
}
