use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job");
            job();
        });

        Worker {
            id,
            thread,
        }
    }
}

struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {
    fn new(size: usize) -> Self {
        if size < 1 {
            panic!("Pool capacity has to be greater than 0!");
        }

        let (sender, receiver) = mpsc::channel();
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(Worker::new(i, receiver));
        }

        WorkerPool {
            workers,
            sender,
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    fn join(self) {
        for worker in self.workers {
            worker.thread.join().unwrap();
        }
    }
}

fn hello() {
    std::thread::sleep(Duration::from_secs(2));
    println!("hello");
}

fn world() {
    std::thread::sleep(Duration::from_secs(3));
    println!("world");
}

fn sign() {
    println!("!");
}

fn main() {
    let pool = WorkerPool::new(2);
    pool.execute(hello);
    pool.execute(world);
    pool.execute(sign);
    pool.join();
}
