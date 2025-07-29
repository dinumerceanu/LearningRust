use std::io;

fn is_armstrong_number(num: u32) -> bool {
    let mut copy = num;
    let mut sum = 0;
    let mut digits = 0;

    while copy > 0 {
        digits += 1;
        copy /= 10;
    }
    
    copy = num;
    while copy > 0 {
        let digit = copy % 10;
        sum += digit.pow(digits);
        copy /= 10;
    }

    if sum == num {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Eroare la citire");

    let mut number = -1;

    if let Ok(num) = input.trim().parse::<i32>() {
        number = num;
    }

    println!("{}", is_armstrong_number(number as u32))
}
