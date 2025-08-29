use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn handle_client(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Cliend disconnected");
                break;
            },
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {msg}");

                if let Err(e) = stream.write_all(b"Message received!\n") {
                    eprintln!("Error at writing response: {e}");
                    break;
                }
            },
            Err(e) => {
                eprintln!("Error at reading");
                break;
            }
        }
    }
}

pub fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server is listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr()?);
                std::thread::spawn(move || {handle_client(&stream);});
            },
            Err(e) => {
                eprintln!("Connection error: {e}");
            }
        }
    }

    Ok(())
}
