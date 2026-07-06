use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(name="Task Manager")]
#[command(about="A Simple CLI Task Manager", long_about = None)]
pub(crate) struct Cli {  // Notice we are using pub(crate)
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Add {
        name: String,
        description: String,
    },
    List,
    Complete {
        id: u64,
    },
    Delete {
        id: u64,
    },
}

