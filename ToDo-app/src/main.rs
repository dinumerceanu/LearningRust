use clap::{Parser, Subcommand};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{fs, io::{self, Write}, vec};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Adds a new task to the list
    Add {
        task_name: String,
        #[arg(value_parser = parse_date)]
        deadline: NaiveDate,
    },
    /// Deletes a task from list
    Delete {
        task_name: String,
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

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task_name: String,
    deadline: NaiveDate,
    completed: bool,
}

impl Task {
    fn new(task_name: String, deadline: NaiveDate) -> Self {
        Task {
            task_name,
            deadline,
            completed: false
        }
    }

    fn mark_complete(&mut self) {
        self.completed = true;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TODOList {
    list: Vec<Task>
}

impl TODOList {
    fn new() -> Self {
        TODOList { list: Vec::new() }
    }

    fn add(&mut self, task: Task) {
        self.list.push(task);
    }

    fn print(&mut self) {
        println!("{:?}", self.list);
    }

    fn remove(&mut self, task_name: String) {
        let n = self.list.len();
        self.list.retain(|task| task.task_name != task_name);

        if n == self.list.len() {
            println!("Task is not in list");
        }
    }

    fn mark_complete(&mut self, task_name: String) {
        for task in &mut self.list {
            if task.task_name.eq(&task_name) {
                task.mark_complete();
            }
        }
    }

    fn save(&self, path: &str) {
        match serde_json::to_string_pretty(&self.list) {
            Ok(json) => {
                if let Err(e) = std::fs::write(path, json) {
                    println!("Failed to write file: {}", e);
                } else {
                    println!("Saved to {}", path);
                }
            }
            Err(e) => println!("Failed to serialize: {}", e),
        }
    }

    fn load(path: &str) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            if contents.trim().is_empty() {
                return TODOList::new();
            }

            match serde_json::from_str::<Vec<Task>>(&contents) {
                Ok(list) => TODOList { list },
                Err(e) => {
                    println!("Failed to parse JSON ({}). Starting with empty list.", e);
                    TODOList::new()
                }
            }
        } else {
            TODOList::new()
        }
    }
}

fn main() {
    let prog_name = env!("CARGO_PKG_NAME");
    let mut list = TODOList::load("tasks.json");

    loop {
        print!("{}> ", prog_name);
        io::stdout().flush().unwrap();

        let mut in_buf = String::new();
        if io::stdin().read_line(&mut in_buf).is_err() {
            println!("Failed to read input");
            continue;
        }

        let line = in_buf.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(mut args) = shlex::split(line) {
            let mut argv = vec![prog_name.to_string()];
            argv.append(&mut args);

            match Args::try_parse_from(&argv) {
                Ok(cli) => match cli.cmd {
                    Commands::Add { task_name, deadline } => 
                        list.add(Task::new(task_name, deadline)),
                    Commands::Print => list.print(),
                    Commands::Delete { task_name } => list.remove(task_name),
                    Commands::Mark { task_name } => list.mark_complete(task_name),
                    Commands::Save => list.save("tasks.json"),
                    Commands::Exit => {list.save("tasks.json"); break;}
                },
                Err(e) => e.print().unwrap()
            }
        } else {
            println!("Invalid quoting in input");
        }
    }
}