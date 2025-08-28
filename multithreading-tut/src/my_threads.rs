use std::thread;
use std::cell::RefCell;

pub fn main_thread() {
    let mut x = 0i128;

    for i in 1..1000 {
        x += i;
    }

    println!("Main thread is finished, x is {x}");
}

pub fn spawn_2() {
    let math_closure = || {
        let mut x = 0i128;

        for i in 1..5_000_000 {
            x += i;
        }

        println!("Worker thread finished");
    };

    let math_closure2 = || {
        let mut x = 0i128;

        for i in 1..50_000_000 {
            x += i;
        }

        println!("Worker thread finished");
    };

    let thread1_handle = thread::spawn(math_closure);
    let thread2_handle = thread::spawn(math_closure2);

    loop {
        if thread1_handle.is_finished() && thread2_handle.is_finished() {
            println!("All workers are done");
            break;
        }
    }
}

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(0);
}

pub fn named_threads() {
    let math_closure = || {
        COUNTER.with(|c| {
            *c.borrow_mut() += 1;
            println!("COUNTER is {} from thread {:?}", c.borrow(), thread::current().name());
        });

        let mut x = 0i128;

        for i in 1..5_000_000 {
            x += i;
        }

        println!("Work finished in {:?} {:?}", thread::current().name(), thread::current().id());
    };

    let math_closure2 = || {
        COUNTER.with(|c| {
            *c.borrow_mut() += 1;
            println!("COUNTER is {} from thread {:?}", c.borrow(), thread::current().name());
        });

        let mut x = 0i128;

        for i in 1..50_000_000 {
            x += i;
        }

        println!("Work finished in {:?} {:?}", thread::current().name(), thread::current().id());
    };

    let thread1_handle = thread::Builder::new().name("thread1".to_string()).spawn(math_closure);
    let thread2_handle = thread::Builder::new().name("thread2".to_string()).spawn(math_closure2);

    loop {
        if thread1_handle.as_ref().unwrap().is_finished() && thread2_handle.as_ref().unwrap().is_finished() {
            println!("All workers are done");
            break;
        }
    }
}
