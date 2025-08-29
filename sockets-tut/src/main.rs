mod server;
mod client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- [server|client]");
        return;
    }

    match args[1].as_str() {
        "server" => {
            if let Err(e) = server::run_server() {
                eprintln!("Server error: {e}");
            }
        }
        "client" => {
            if let Err(e) = client::run_client() {
                eprintln!("Client error: {e}");
            }
        }
        _ => eprintln!("Use `server` or `client` as arg"),
    }
}
