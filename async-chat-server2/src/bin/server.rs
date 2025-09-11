use std::net::SocketAddr;

use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

async fn handle_client(
    mut stream: TcpStream,
    sockaddr: SocketAddr,
    tx: broadcast::Sender<(String, SocketAddr)>,
) -> io::Result<()> {
    let (reader, mut writer) = stream.split();

    let mut buf = BufReader::new(reader);

    let mut rx = tx.subscribe();

    let socket_reader_channel_writer = async move {
        loop {
            let mut line = String::new();
            match buf.read_line(&mut line).await {
                Ok(0) => {
                    println!("Connection terminated by client {sockaddr}");
                    break;
                },
                Ok(_) => {
                    let msg = line.trim();
                    println!("Received from {sockaddr}: {msg}");
                    if let Err(e) = tx.send((msg.into(), sockaddr)) {
                        eprintln!("Error broadcasting: {e}");
                        break;
                    } else {
                        println!("message sent on channel");
                    }
                },
                Err(e) => {
                    eprintln!("Error reading from client: {e}");
                    break;
                }
            }
        }
    };

    let channel_reader_socket_writer = async move {
        loop {
            match rx.recv().await {
                Ok((msg, sender_addr)) => {
                    if sender_addr != sockaddr {
                        if let Err(e) = writer.write_all(format!("{msg}\n").as_bytes()).await {
                            eprintln!("Error writing to socket: {e}");
                            break;
                        } else {
                            println!("msg written to socket");
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error receiving from channel: {e}");
                    break;
                }
            }
        }
    };

    tokio::select! {
        _ = socket_reader_channel_writer => {},
        _ = channel_reader_socket_writer => {},
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on 127.0.0.1:8080...");


    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(64);

    loop {
        let (stream, sockaddr) = listener.accept().await?;

        println!("New connection from {sockaddr}");

        let tx_clone = tx.clone();

        tokio::spawn(async move {
            handle_client(stream, sockaddr, tx_clone).await
        });
    }
}
