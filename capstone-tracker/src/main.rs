
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};

mod models; // This tells rust to look up for a models.rs file
mod storage;
use models::{Task, TaskStatus};// This brings them into scope so we don't have to type models::Task



// The struct which represents the entire CLI application
#[derive(Parser, Debug)]
#[command(name = "Task Manager")]
#[command(about = "A Simple CLI Task Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


// The enum represents the different commands the user can type
#[derive(Subcommand, Debug)]
enum Commands {
    // Add a new task
    Add {
        name: String,
        description: String,
    },
    /// List all tasks
    List,
}



// Telling Rust main can fall and return an IO error
fn main() -> Result<(), std::io::Error> {

    let cli = Cli::parse();
    // println!("Parsed CLI : {:#?}", cli);
    
    let mut task_list = storage::load_tasks();

    match cli.command {
        Commands::Add {name, description} => {
            let new_task = Task::new(name, description);
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

                println!("[{}] {} - {}", status_string, task.name, task.description);
            }
        }
    }

    storage::save_tasks(&task_list)?;
    // Return Ok at the very end to signal the program finished succesfully
    Ok(())
    
    
}
