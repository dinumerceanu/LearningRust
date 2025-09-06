use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use std::{
    net::SocketAddr,
};

async fn handle_client(mut socket: TcpStream, addr: SocketAddr, tx_clone: broadcast::Sender<(String, SocketAddr)>, mut rx_clone: broadcast::Receiver<(String, SocketAddr)>) -> io::Result<()> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => {
                        break;
                    },
                    Ok(_) => {
                        let msg = line.trim().to_string();
                        println!("Broadcasting {msg}");
                        if let Err(e) = tx_clone.send((msg, addr)) {
                            eprintln!("Error writing to broadcast channel: {e}");
                        }
                        line.clear();
                    },
                    Err(e) => {
                        eprintln!("Error reading from client {addr}: {e}");
                        break;
                    }
                }
            }
            result = rx_clone.recv() => {
                match result {
                    Ok((msg, sender_addr)) => {
                        if sender_addr != addr {
                            let mut msg = msg;
                            msg.push('\n');
                            let actual_msg = format!("[{}]: {}", sender_addr, msg);

                            if let Err(e) = writer.write_all(actual_msg.as_bytes()).await {
                                eprintln!("Error writing to client {}: {}", addr, e);
                                break;
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error at something {e}");
                    }
                }
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on 127.0.0.1:8080");

    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(100);

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("New connection from {:?}", addr);

        let tx_clone = tx.clone();
        let rx_clone = tx.subscribe(); 

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, tx_clone, rx_clone).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
            println!("Client disconnected: {}", addr);
        });
    }
}
