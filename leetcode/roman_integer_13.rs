use std::io;
use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let mut numbers: HashMap<char, i32> = HashMap::new();

    numbers.insert('I', 1);
    numbers.insert('V', 5);
    numbers.insert('X', 10);
    numbers.insert('L', 50);
    numbers.insert('C', 100);
    numbers.insert('D', 500);
    numbers.insert('M', 1000);

    let mut string = s.trim().chars().rev();
    let mut total = 0;
    let mut prev = 0;

    for c in string {
        let num = numbers.get(&c).unwrap();
        if *num >= prev {
            total += *num;
        } else {
            total -= *num;
        }
        prev = *num;
    }

    total
}

fn main() {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Eroare la citire");

    println!("{}", roman_to_int(input));
}