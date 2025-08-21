use clap::{Parser, Subcommand, ArgGroup};
use chrono::NaiveDate;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Adds a new task to the list
    Add {
        task_name: String,

        #[arg(value_parser = parse_date)]
        deadline: NaiveDate,

        /// Set priority: low, medium, high
        #[arg(long = "set-priority", value_parser = ["low", "medium", "high"], default_value = "low")]
        priority: String,
    },
    /// Deletes a task from list
    Delete {
        task_name: Option<String>,

        #[arg(long, short, help = "Deletes without confirmation")]
        force: bool,

        #[arg(long, short, help = "Deletes all tasks")]
        all: bool
    },
    /// Marks a task as completed
    Mark {
        /// Optional task name (ignored if --all is set)
        task_name: Option<String>,

        #[arg(long, short, help = "Marks a task uncompleted")]
        uncomplete: bool,

        #[arg(long, short, help = "Marks all tasks completed")]
        all: bool
    },
    /// Prints the list
    #[command(group(
        ArgGroup::new("print_mode")
            .args(&["completed", "pending", "sort", "all"])
            .required(true).multiple(true)
    ))]
    Print {
        #[arg(long, short, help = "Prints only completed tasks")]
        completed: bool,

        #[arg(long, short, help = "Prints only uncompleted tasks")]
        pending: bool,

        /// Sorting criteria: "deadline", "name", "status"
        #[arg(long, value_parser = ["deadline", "name", "status"])]
        sort: Option<String>,

        #[arg(long, short, conflicts_with = "pending", conflicts_with = "completed", conflicts_with = "sort", help = "Prints all tasks")]
        all: bool
    },
    /// Saves the list to a json object locally
    Save,
    /// Exists the program
    Exit
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%d.%m.%Y")
        .map_err(|_| format!("Invalid date. Use 'DD-MM-YYYY' format"))
}
