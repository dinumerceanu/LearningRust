use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Get the value for a given key
    Get {
        key: String,
    },
    /// Set a key-value pair
    Set {
        key: String,
        value: String,
        /// Optional flag
        #[arg(long)]
        is_true: bool,
    },
    Quit
}

fn get_something(key: String) {
    println!("Getting key: {}", key);
}

fn set_something(key: String, value: String, is_true: bool) {
    println!(
        "Setting key: {}, value: {}, is_true: {}",
        key, value, is_true
    );
}

fn main() {
    let prog_name = env!("CARGO_PKG_NAME");

    loop {
        // Prompt
        print!("{}> ", prog_name);
        io::stdout().flush().unwrap();

        // Citește input
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            println!("Failed to read input");
            continue;
        }

        let line = buf.trim();
        if line.is_empty() {
            continue;
        }

        // Parsează linia ca și cum ar fi argv
        if let Some(mut args) = shlex::split(line) {
            println!("args: {:?}", args);
            // primul argument = numele programului (argv[0])
            let mut argv = vec![prog_name.to_string()];
            argv.append(&mut args);
            println!("argv: {:?}", argv);

            // încearcă parsarea cu Clap
            match Args::try_parse_from(&argv) {
                Ok(cli) => match cli.cmd {
                    Commands::Get { key } => get_something(key),
                    Commands::Set { key, value, is_true } => {
                        set_something(key, value, is_true)
                    },
                    Commands::Quit => {
                        println!("exiting...");
                        break;
                    }
                },
                Err(e) => {
                    // Clap știe să printeze usage și help frumos
                    e.print().unwrap();
                }
            }
        } else {
            println!("Invalid quoting in input");
        }
    }
}
