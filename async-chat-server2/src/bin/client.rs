use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream, select,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:8080").await?;

    let (reader, mut writer) = client.split();

    let stdin_reader_future = async move {
        let mut reader = BufReader::new(io::stdin());

        loop {
            let mut line = String::new();
            
            match reader.read_line(&mut line).await {
                Ok(n) if n < 2 => {
                    println!("Cannot send empty message. Retry:");
                    continue;
                },
                Ok(_) => {
                    if let Err(e) = writer.write_all(line.as_bytes()).await {
                        eprintln!("Error writing to server: {e}");
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Error reading from stdin: {e}");
                    break;
                }
            }
        }

    };
    
    let server_reader_future = async move {
        let mut reader = BufReader::new(reader);
        
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    println!("Connection terminated by the server");
                    break;
                },
                Ok(_) => {
                    let msg = line.trim();
                    println!("Received from server: {msg}");
                },
                Err(e) => {
                    eprintln!("Error reading from server: {e}");
                    break;
                }
            }
        }
    };

    select! {
        _ = stdin_reader_future => {},
        _ = server_reader_future => {},
    };

    std::process::exit(0);
}
