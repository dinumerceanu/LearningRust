use std::env;

use crate::{client::run_client, server::run_server};

mod thread_pool;
mod server;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- [server/client]");
        return;
    }

    match args[1].as_str() {
        "server" => {
            println!("Starting server...");
            run_server();
        },
        "client" => {
            println!("Starting client...");
            run_client();
        },
        _ => {
            println!("unknown command...");
        }
    }
}
