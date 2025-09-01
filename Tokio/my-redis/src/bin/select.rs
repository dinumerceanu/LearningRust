// use tokio::sync::oneshot;

// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     tokio::spawn(async {
//         let _ = tx1.send("one");
//     });

//     tokio::spawn(async {
//         let _ = tx2.send("two");
//     });

//     tokio::select! {
//         val = rx1 => {
//             println!("rx1 completed first with {:?}", val);
//         },
//         val = rx2 => {
//             println!("rx2 completed first with {:?}", val);
//         }
//     }
// }


// use std::thread;
// use std::time::Duration;

// use tokio::net::TcpStream;
// use tokio::sync::oneshot;

// #[tokio::main]
// async fn main() {
//     let (tx, rx) = oneshot::channel();

//     // Spawn a task that sends a message over the oneshot
//     tokio::spawn(async move {
//         thread::sleep(Duration::from_millis(100));
//         tx.send("done");
//     });

//     tokio::select! {
//         socket = TcpStream::connect("localhost:3465") => {
//             println!("Socket connected {:?}", socket);
//         }
//         msg = rx => {
//             println!("received message first {:?}", msg);
//         }
//     }
// }

// fn fib(n: usize) -> usize {
//     match n {
//         0 => 0,
//         1 => 1,
//         _ => fib(n - 1) + fib(n - 2)
//     }
// }

// async fn computation1() -> String {
//     fib(5).to_string()
// }

// async fn computation2() -> String {
//     fib(10).to_string()
// }

// #[tokio::main]
// async fn main() {
//     let out = tokio::select! {
//         res1 = computation1() => res1,
//         res2 = computation2() => res2,
//     };

//     println!("Got = {}", out);
// }




// use std::{thread, time::Duration};

// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let (tx1, mut rx1) = mpsc::channel(128);
//     let (tx2, mut rx2) = mpsc::channel(128);
//     let (tx3, mut rx3) = mpsc::channel(128);

//     for _ in 0..2 {
//         let tx1 = tx1.clone();
//         let tx2 = tx2.clone();
//         let tx3 = tx3.clone();

//         tokio::spawn(async move {
//             thread::sleep(Duration::from_secs(1));
//             tx1.send(10).await.unwrap();
//         });

//         tokio::spawn(async move {
//             thread::sleep(Duration::from_secs(1));
//             tx2.send(20).await.unwrap();
//         });

//         tokio::spawn(async move {
//             thread::sleep(Duration::from_secs(1));
//             tx3.send(30).await.unwrap();
//         });
//     }

//     drop(tx1);
//     drop(tx2);
//     drop(tx3);
    
//     loop {
//         let msg = tokio::select! {
//             Some(msg) = rx1.recv() => msg,
//             Some(msg) = rx2.recv() => msg,
//             Some(msg) = rx3.recv() => msg,
//             else => { break }
//         };

//         println!("Got {:?}", msg);
//     }

//     println!("All channels have been closed.");
// }


async fn action(input: Option<i32>) -> Option<String> {
    // If the input is `None`, return `None`.
    // This could also be written as `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    // async logic here
    Some(i.to_string())
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);
    
    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);
    
    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });
    
    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    // `.set` is a method on `Pin`.
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        }
    }
}
