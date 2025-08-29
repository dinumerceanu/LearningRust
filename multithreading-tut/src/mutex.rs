use std::{ops::AddAssign, result, sync::Mutex, thread::ScopedJoinHandle, time::Duration};

pub fn test_mutex() {
    let mut score = Mutex::new(0u16);

    // let unlocked_data = score.lock();
    // let mut data = unlocked_data.unwrap();

    // data.add_assign(1);

    // println!("{:?}", data);

    let my_func = || {
        println!("Thread 1 is waiting for mutex lock...");
        let mut data = score.lock().unwrap();
        
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}");
            std::thread::sleep(Duration::from_millis(400));
        }
    };

    let my_func2 = || {
        println!("Thread 2 is waiting for mutex lock...");

        let inner_scope_thread = || {
            let mut data = score.lock().unwrap();
            // drop(data); // if we don't drop the guard before the panic, the next thread cannot access the data
            // panic!("Thread 2 panicked");
            
            for i in 1..10 {
                if i == 5 {
                    drop(data);
                    panic!("i is 5 in thread 2");
                }
                data.add_assign(i);
                println!("Thread 2 is adding {i}");
            }
        };

        let result = std::panic::catch_unwind(inner_scope_thread);

        if result.is_err() {
            println!("Thread 2 panicked, but we caught the panic");
        }
    };

    let my_func3 = || {
        loop {
            println!("Thread 3 is waiting for mutex lock...");

            let guard = score.try_lock();

            if guard.is_ok() {
                let mut data = guard.unwrap();
                for i in 1..10 {
                    data.add_assign(i);
                    println!("Thread 3 is adding {i}");
                }
                break;
            }
            std::thread::sleep(Duration::from_millis(300));
        }
    };

    std::thread::scope(|scope| {
        let handle2 = scope.spawn(my_func2);
        let handle1 = scope.spawn(my_func);
        let handle3 = scope.spawn(my_func3);

        // if handle2.is_err() {
        //     println!("Thread 2 had an error and panicked");
        // }
    });

    // println!("{:?}", score.lock().unwrap());

    match score.lock() {
        Ok(guard) => {
            println!("Final score (safe) is {:?}", *guard);
        },
        Err(poisoned) => {
            println!("Mutex is poisoned! Recovering value...");
            let guard = poisoned.into_inner();
            println!("Recovered score: {:?}", *guard);
        }
    }
}
