use std::time::Duration;
use tokio::time::sleep;

pub async fn run() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function");
    let s1 = read_from_db().await;
    println!("[{i}] First result: {s1}");
    let s2 = read_from_db().await;
    println!("[{i}] Second result: {s2}");
}

async fn read_from_db() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB ressult".into()
}
