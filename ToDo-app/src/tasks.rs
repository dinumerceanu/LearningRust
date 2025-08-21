use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::{fs, io::{self, Write}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
pub struct Task {
    task_name: String,
    deadline: NaiveDate,
    completed: bool,
    priority: Priority
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.completed, other.completed) {
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            _ => {
                let self_prio = match self.priority {
                    Priority::High => 3,
                    Priority::Medium => 2,
                    Priority::Low => 1,
                };
                let other_prio = match other.priority {
                    Priority::High => 3,
                    Priority::Medium => 2,
                    Priority::Low => 1,
                };
                match self_prio.cmp(&other_prio) {
                    std::cmp::Ordering::Equal => {
                        self.deadline.cmp(&other.deadline)
                    }
                    ord => ord.reverse(),
                }
            }
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Task {
    pub fn new(task_name: String, deadline: NaiveDate, priority: Priority) -> Self {
        Task {
            task_name,
            deadline,
            completed: false,
            priority
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

            if task.deadline < chrono::Local::now().date_naive() {
                println!("Cannot add task '{}' with a past deadline.", task.task_name);
                return;
            } 

            println!("Added task: {} (deadline: {}, priority: {:?})", task.task_name, task.deadline, task.priority);
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
                "priority" => tasks.sort_by_key(|t| std::cmp::Reverse(t.priority.clone())),
                _ => {}
            }
        }

        if all {
            tasks.sort();
        }

        if tasks.is_empty() {
            println!("No tasks found");
        }

        for task in tasks {
            println!(
                "[{}] {} (deadline: {}, priority: {:?})",
                if task.completed { "X" } else { " " },
                task.task_name,
                task.deadline,
                task.priority
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

    pub fn edit(&mut self,  task_name: String, new_name: Option<String>, new_deadline: Option<NaiveDate>, new_priority: Option<String>) {
        if let Some(found_task) = self.list.iter_mut().find(|x| x.task_name.eq(&task_name)) {
            if let Some(name) = new_name {
                found_task.task_name = name;
            }

            if let Some(ddl) = new_deadline {
                found_task.deadline = ddl;
            }

            if let Some(prio) = new_priority {
                match prio.to_lowercase().as_str() {
                    "low" => found_task.priority = Priority::Low,
                    "medium" => found_task.priority = Priority::Medium,
                    "high" => found_task.priority = Priority::High,
                    _ => println!("Invalid priority, use: Low, Medium, High"),
                }
            }
        } else {
            println!("Task '{}' not found", task_name);
        }
    }
}
