use std::io;

fn collatz(n: u64) -> Option<u64> {
    if n <= 0  {
        return None;
    }
    let mut num = n;
    let mut steps = 0;

    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = num * 3 + 1;
        }
        steps += 1;
    }

    Some(steps)
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Eroare la citire");

    if let Ok(num) = input.trim().parse::<u64>() {
        println!("{:?}", collatz(num));
    } else {
        println!("Nu ai introdus un numar");
    }
}