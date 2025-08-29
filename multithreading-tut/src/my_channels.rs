use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

pub fn test_channels() {
    let (tx, rx) = channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        for i in (1..20).step_by(2) {
            tx.send(i * i).unwrap();
            thread::sleep(Duration::from_millis(15));
        }
    });

    thread::spawn(move || {
        for i in (0..20).step_by(2) {
            tx1.send(i * i).unwrap();
            thread::sleep(Duration::from_millis(15));
        }
    });

    for receiver in rx.iter() {
        println!("received: {}", receiver);
    }
}
