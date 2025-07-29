use std::io;

fn reverse(input: &str) -> String {
    let mut res = String::new();
    let mut stack: Vec<char> = Vec::new();

    for elem in input.chars() {
        stack.push(elem);
    }

    // while !stack.is_empty() {
    //     res.push(stack.pop().unwrap());
    // }

    while let Some(element) = stack.pop() {
        res.push(element);
    }

    res
}

fn main() {
    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("eroare la citire");

    let res = reverse(&string);

    println!("{}", res);
}