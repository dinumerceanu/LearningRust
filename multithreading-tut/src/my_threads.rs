use std::thread;

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