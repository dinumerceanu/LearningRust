use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        println!("New connection from {:?}", socket.peer_addr());

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Connection from {:?} terminated", socket.peer_addr());
                        break;
                    },
                    Ok(n) => {
                        let s = String::from_utf8_lossy(&buf[..n]);
                        let trimmed_s = s.trim();
                        println!("Received from {:?}: {:?}", socket.peer_addr(), trimmed_s);
                        socket.write(&buf[..n]).await;
                    },
                    Err(e) => {
                        eprintln!("Error readinng from {:?}: {}", socket.peer_addr(), e);
                    }
                }
            }
        });
    }
}
