struct Person {
    first_name: String,
}

pub fn scoped_threads_demo() {
    let age: i32 = 20;
    let person = Person {first_name: String::from("Mike")};

    let print_info = || {
        println!("This is from closure");
        println!("Age is {age}");
        println!("Name is {}", person.first_name);
    };

    std::thread::scope(|scope| {
        scope.spawn(print_info);
    });

    // std::thread::spawn(print_info);

    println!("Control back to main");
    println!("Age is {age}");
    println!("Name is {}", person.first_name);
}