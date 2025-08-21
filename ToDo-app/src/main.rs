use clap::Parser;
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
                    Commands::Add { task_name, deadline } => 
                        list.add(Task::new(task_name, deadline)),
                    Commands::Print => list.print(),
                    Commands::Delete { task_name, force } => list.delete(task_name, force),
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
