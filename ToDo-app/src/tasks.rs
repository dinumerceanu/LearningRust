use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::{fs, future::Pending, io::{self, Write}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    task_name: String,
    deadline: NaiveDate,
    completed: bool,
}

impl Task {
    pub fn new(task_name: String, deadline: NaiveDate) -> Self {
        Task {
            task_name,
            deadline,
            completed: false
        }
    }

    fn mark_complete(&mut self) {
        self.completed = true;
    }

    fn mark_uncomplete(&mut self) {
        self.completed = false;
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.task_name == other.task_name
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TODOList {
    list: Vec<Task>
}

impl TODOList {
    pub fn new() -> Self {
        TODOList { list: Vec::new() }
    }

    pub fn add(&mut self, task: Task) {
        if !self.list.contains(&task) {
            self.list.push(task);
        } else {
            println!("Task already exists");
        }
    }

    pub fn print(&self, completed: bool, pending: bool, sort: Option<String>, all: bool) {
        let mut tasks: Vec<Task> = self
            .list
            .iter()
            .cloned()
            .filter(|t| {
                if all {
                    true
                } else if completed {
                    t.completed
                } else if pending {
                    !t.completed
                } else {
                    true
                }
            })
            .collect();

        if let Some(sort_key) = sort {
            match sort_key.as_str() {
                "deadline" => tasks.sort_by_key(|t| t.deadline),
                "name"     => tasks.sort_by_key(|t| t.task_name.clone()),
                "status"   => tasks.sort_by_key(|t| t.completed),
                _ => {}
            }
        }

        for task in tasks {
            println!(
                "[{}] {} (deadline: {})",
                if task.completed { "X" } else { " " },
                task.task_name,
                task.deadline
            );
        }
    }

    pub fn delete(&mut self, task_name: String, force: bool) {
        if let Some(pos) = self.list.iter().position(|t| t.task_name == task_name) {
            if !force {
                print!("Are you sure you want to delete task: {}?[y/n]: ", task_name);
                io::stdout().flush().unwrap();

                let mut decision = String::new();
                if io::stdin().read_line(&mut decision).is_ok() {
                    match decision.trim() {
                        "y" | "Y" => {
                            self.list.remove(pos);
                            println!("Task deleted.");
                        }
                        "n" | "N" => println!("Deletion cancelled."),
                        _ => println!("Invalid input, task not deleted."),
                    }
                }
            } else {
                self.list.remove(pos);
            }
        } else {
            println!("Task is not in list.");
        }
    }

    pub fn delete_all(&mut self, force: bool) {
        if !force {
            print!("Are you sure you want to delete all tasks? [y/n]: ");
            io::stdout().flush().unwrap();

            let mut decision = String::new();
            if io::stdin().read_line(&mut decision).is_ok() {
                match decision.trim() {
                    "y" | "Y" => {
                        self.list = Vec::new();
                        println!("All tasks deleted.");
                    }
                    "n" | "N" => println!("Deletion cancelled."),
                    _ => println!("Invalid input, tasks not deleted."),
                }
            }
        } else {
            self.list = Vec::new();
            println!("All tasks deleted.");
        }
    }

    pub fn mark(&mut self, task_name: String, uncomplete: bool) {
        for task in &mut self.list {
            if task.task_name.eq(&task_name) {
                if !task.completed && !uncomplete{
                    task.mark_complete();  
                    println!("Task {} was marked completed.", task_name);
                } else if task.completed && !uncomplete {
                    println!("Task is already completed.");
                } else if task.completed && uncomplete {
                    task.mark_uncomplete();
                    println!("Task {} was marked uncompleted.", task_name);
                } else {
                    println!("Task is not marked as completed.");
                }
                return;
            }
        }

        println!("Task is not in list");
    }

    pub fn mark_all(&mut self, uncomplete: bool) {
        if self.list.is_empty() {
            println!("No tasks to be marked.");
            return;
        }
        
        for task in &mut self.list {
            if !task.completed && !uncomplete {
                task.completed = true;
            } else if task.completed && uncomplete {
                task.completed = false;
            }
        }

        if uncomplete {
            println!("All tasks marked uncompleted.");
        } else {
            println!("All tasks marked completed.");
        }
    }

    pub fn save(&self, path: &str) {
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

    pub fn load(path: &str) -> Self {
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
