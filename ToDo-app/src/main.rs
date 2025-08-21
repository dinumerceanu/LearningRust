use clap::{Parser};
use std::{io::{self, Write}, vec};

mod tasks;
use tasks::*;

mod commands;
use commands::*;

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
                    Commands::Add { task_name, deadline, priority } => {
                        let priority = match priority.as_str() {
                            "low" => Priority::Low,
                            "medium" => Priority::Medium,
                            "high" => Priority::High,
                            _ => Priority::Low
                        };
                        list.add(Task::new(task_name, deadline, priority));
                    },
                    Commands::Print {completed, pending, sort, all} => {
                        list.print(completed, pending, sort, all);
                    },
                    Commands::Delete { task_name, force , all} => {
                        if all {
                            list.delete_all(force);
                        } else if let Some(name) = task_name {
                            list.delete(name, force);
                        }
                    }
                    Commands::Mark { task_name , uncomplete, all} => {
                        if all {
                            list.mark_all(uncomplete);
                        } else if let Some(name) = task_name {
                            list.mark(name, uncomplete);
                        }
                    }
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
