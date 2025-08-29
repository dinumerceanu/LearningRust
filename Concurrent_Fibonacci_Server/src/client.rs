use std::io::{self, Write, Read};
use std::net::TcpStream;

pub fn run_client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server!");

    loop {
        let mut input = String::new();
        print!("Message > ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            println!("Disconnecting...");
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("Server: {}", response);
    }

    Ok(())
}
