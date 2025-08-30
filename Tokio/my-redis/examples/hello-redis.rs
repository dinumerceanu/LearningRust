use mini_redis::{client, Result};
// use tokio::time::sleep;
// use std::time::Duration;

// async fn say_world() {
//     sleep(Duration::from_secs(5)).await;
//     println!("world");
// }

// async fn say_world2() {
//     sleep(Duration::from_secs(2)).await;
//     println!("world2");
// }

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result  = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    // let task = tokio::spawn(say_world());
    // let task2 = tokio::spawn(say_world2());

    // println!("hello");

    // println!("RUST");

    // task.await.unwrap();
    // task2.await.unwrap();


    Ok(())
}
