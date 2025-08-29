use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<Option<thread::JoinHandle<()>>>,
}

impl ThreadPool {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers: Vec<Option<thread::JoinHandle<()>>> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            let rx = Arc::clone(&rx);
            let handle = thread::spawn(move || loop {
                let message = rx.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {i} started the job");
                        let x = job();
                        println!("{:?}", x);
                    },
                    Err(e) => {
                        println!("Worker {i} shutting down...Encountering error: {e}");
                        break;
                    }
                }
            });

            workers.push(Some(handle));
        }

        ThreadPool {
            sender: Some(tx),
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        while let Some(handle) = self.workers.pop() {
            handle.unwrap().join().unwrap();
        }
    }
}
