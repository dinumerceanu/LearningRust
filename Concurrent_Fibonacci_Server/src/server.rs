use crate::thread_pool::ThreadPool;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc}, thread,
};

fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

pub fn handle_client(mut stream: TcpStream, pool: &ThreadPool) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {msg}");

                let num = msg.trim().parse::<usize>();

                if let Ok(num) = num {
                    let (tx, rx) = mpsc::channel();
                    let job = move || {
                        let fib_res = fib(num);
                        tx.send(fib_res).unwrap();
                    };

                    pool.execute(job);

                    if let Ok(fib_res) = rx.try_recv() {
                        let response = format!("fib({num}) = {fib_res}\n"); 
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            eprintln!("Error writing response: {e}");
                            break;
                        }
                    }
                } else {
                    let err_msg = format!("{msg} is not a valid number\n");
                    if let Err(e) = stream.write_all(err_msg.as_bytes()) {
                        eprintln!("Error writing response: {e}");
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error at reading: {e}");
                break;
            }
        }
    }
}

pub fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server is listening on 127.0.0.1:8080");

    let pool = Arc::new(ThreadPool::new(2));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected from {}", stream.peer_addr()?);

                let pool = Arc::clone(&pool);
                thread::spawn(move || {handle_client(stream, &pool)});
            }
            Err(e) => {
                eprintln!("Connection error: {e}");
            }
        }
    }

    Ok(())
}
