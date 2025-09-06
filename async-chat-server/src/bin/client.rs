use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use std::io::Write;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    println!("Connected to server!");

    let (mut stream_reader, mut stream_writer) = stream.into_split();

    let stdin_reader_future = async move {
        loop {
            print!("async-chat> ");
            std::io::stdout().flush().unwrap();

            let mut buf = vec![0; 1024];
            let res = io::stdin().read(&mut buf).await;

            if let Ok(n) = res {
                if n < 2 {
                    continue;
                }

                match stream_writer.write_all(&buf[..n]).await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error writing to the server: {e}");
                    }
                }
                
            }
        }
    };

    let server_reader_future = async move {
        loop {
            let mut buf = vec![0; 1024];
            let res = stream_reader.read(&mut buf).await;

            match res {
                Ok(0) => {
                    print!("\x1B[2K\x1B[1G");
                    std::io::stdout().flush().unwrap();
                    println!("Connection terminated by the server");
                    break;
                },
                Ok(n) => {
                    let s = String::from_utf8_lossy(&buf[..n]);
                    let trimmed_s = s.trim();
                    print!("\x1B[2K\x1B[1G");
                    std::io::stdout().flush().unwrap();
                    println!("Received from server: {:?}", trimmed_s);
                    print!("async-chat> ");
                    std::io::stdout().flush().unwrap();
                },
                Err(e) => {
                    eprintln!("Error reading from server: {e}");
                    break;
                }
            } 
        }
    };

    tokio::select! {
        _ = stdin_reader_future => {},
        _ = server_reader_future => {},
    }

    std::process::exit(0);
}
