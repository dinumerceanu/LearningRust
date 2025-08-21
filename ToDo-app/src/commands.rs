use clap::{Parser, Subcommand};
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
    },
    /// Deletes a task from list
    Delete {
        task_name: String,
        #[arg(long, short, help = "Delete without confirmation")]
        force: bool
    },
    /// Marks a task as completed
    Mark {
        task_name: String,
    },
    /// Prints the list
    Print,
    /// Saves the list to a json object locally
    Save,
    /// Exists the program
    Exit
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%d.%m.%Y")
        .map_err(|_| format!("Invalid date. Use 'DD-MM-YYYY' format"))
}
